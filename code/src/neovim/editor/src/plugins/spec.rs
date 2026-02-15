// ── Plugin Spec System ──
//
// Two macros to define plugins:
//
//   lua_spec!("plugin.lua")
//   lua_spec!("plugin.lua", &[("VAR", "value")])
//   lua_spec!(raw r#"{ "author/plugin", lazy = true }"#)
//
// One macro for section extraction:
//
//   lua_section!("sub_plugin.lua", "spec")
//   lua_section!("sub_plugin.lua", "setup", &[("VAR", "value")])

pub struct SpecInfo {
    pub spec: &'static str,
    pub file: &'static str,
    pub spec_line: u32,
}

#[macro_export]
macro_rules! lua_spec {
    // Inline Lua string (legacy)
    (raw $spec:expr) => {
        $crate::plugins::spec::SpecInfo { spec: $spec, file: file!(), spec_line: line!() }
    };
    // File, no args
    ($path:literal) => {{
        let content = include_str!($path);
        let parsed = $crate::plugins::spec::parse_lua(content, &[], $path, file!(), line!());
        let tagged = format!("(function()\n--@src:{}\n{parsed}\nend)()", $path);
        $crate::plugins::spec::SpecInfo { spec: Box::leak(tagged.into_boxed_str()), file: file!(), spec_line: line!() }
    }};
    // File with args
    ($path:literal, $args:expr) => {{
        let content = include_str!($path);
        let parsed = $crate::plugins::spec::parse_lua(content, $args, $path, file!(), line!());
        let tagged = format!("(function()\n--@src:{}\n{parsed}\nend)()", $path);
        $crate::plugins::spec::SpecInfo { spec: Box::leak(tagged.into_boxed_str()), file: file!(), spec_line: line!() }
    }};
}

#[macro_export]
macro_rules! lua_section {
    ($path:literal, $section:literal) => {
        $crate::plugins::spec::extract_section(($path, include_str!($path)), $section, &[])
    };
    ($path:literal, $section:literal, $args:expr) => {
        $crate::plugins::spec::extract_section(($path, include_str!($path)), $section, $args)
    };
}

// ── Template Parser ──
//
// Replaces `--[[@rs $VAR ]]` markers in Lua files with values from Rust.
// Reports to Neovim if a `$VAR` has no matching arg.

pub fn parse_lua(content: &str, args: &[(&str, &str)], lua_path: &str, rust_file: &str, rust_line: u32) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;
    let arg_names: Vec<&str> = args.iter().map(|(n, _)| *n).collect();

    while let Some(start) = rest.find("--[[@rs") {
        result.push_str(&rest[..start]);

        let after_marker = &rest[start + 7..];
        if let Some(end) = after_marker.find("]]") {
            let template = after_marker[..end].trim();

            if check_unresolved(template, &arg_names, lua_path, rust_file, rust_line) {
                let mut expanded = template.to_string();
                for (name, val) in args {
                    expanded = expanded.replace(&format!("${name}"), val);
                }

                let marker_newlines = rest[start..start + 7 + end + 2].chars().filter(|&c| c == '\n').count();
                let expanded_newlines = expanded.chars().filter(|&c| c == '\n').count();

                result.push_str(&expanded);

                // Multi-line expansion shifts lines — re-anchor
                if expanded_newlines > marker_newlines {
                    let consumed = content.len() - after_marker[end + 2..].len();
                    let next_original_line = content[..consumed].chars().filter(|&c| c == '\n').count() + 1;
                    result.push_str(&format!("\n--@src:{lua_path}:{next_original_line}\n"));
                }
            }

            rest = &after_marker[end + 2..];
        } else {
            result.push_str(&rest[start..start + 7]);
            rest = &rest[start + 7..];
        }
    }

    result.push_str(rest);
    result
}

fn check_unresolved(template: &str, arg_names: &[&str], lua_path: &str, rust_file: &str, rust_line: u32) -> bool {
    let mut scan = template;
    while let Some(pos) = scan.find('$') {
        let var_end = scan[pos + 1..].find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(scan.len() - pos - 1);
        let var_name = &scan[pos + 1..pos + 1 + var_end];
        if !var_name.is_empty() && !arg_names.contains(&var_name) {
            let msg = format!(
                "[Abstract] Unresolved variable `${var_name}`\n  \
                 --> template: {lua_path}\n  \
                 --> called from: {rust_file}:{rust_line}\n  \
                 Hint: pass (\"{var_name}\", \"value\") in the args",
            );
            tracing::error!("{msg}");
            crate::utils::trace::vim_notify(&msg, crate::utils::trace::NotifyLevel::Error);
            return false;
        }
        scan = &scan[pos + 1 + var_end..];
    }
    true
}

// ── Source Resolution ──
//
// Scans `--@src:path` or `--@src:path:offset` markers in the composed
// spec string and maps Lua error lines back to the original .lua file.

pub struct ResolvedSource {
    pub relative_path: String,
    pub source_line: usize,
}

pub fn resolve_source(spec: &str, lua_error_line: usize) -> Option<ResolvedSource> {
    let mut last_path: Option<String> = None;
    let mut last_line: usize = 0;
    let mut last_offset: usize = 0;

    for (i, line) in spec.lines().enumerate() {
        let line_num = i + 1;
        if line_num > lua_error_line {
            break;
        }
        if let Some(marker_pos) = line.find("--@src:") {
            let rest = &line[marker_pos + 7..];
            let rest = rest.trim_end();
            let parts: Vec<&str> = rest.rsplitn(2, ':').collect();
            if parts.len() == 2 {
                if let Ok(offset) = parts[0].parse::<usize>() {
                    last_path = Some(parts[1].to_string());
                    last_offset = offset;
                } else {
                    last_path = Some(rest.to_string());
                    last_offset = 0;
                }
            } else {
                last_path = Some(rest.to_string());
                last_offset = 0;
            }
            last_line = line_num;
        }
    }

    last_path.map(|path| ResolvedSource {
        relative_path: path,
        source_line: if last_offset > 0 {
            last_offset + (lua_error_line - last_line) - 1
        } else {
            lua_error_line - last_line
        },
    })
}

pub fn resolve_full_path(rust_file: &str, relative_lua_path: &str) -> String {
    match rust_file.rfind('/') {
        Some(pos) => format!("{}/{}", &rust_file[..pos], relative_lua_path),
        None => relative_lua_path.to_string(),
    }
}

// ── Section Extraction ──
//
// Extracts content between `--@section` and `--@end` markers.
// Used for splitting sub-plugin files into spec + setup parts.
// Injects `--@src:path:offset` for error tracing and resolves
// any `--[[@rs $VAR ]]` templates if args are provided.

pub fn extract_section((path, content): (&str, &str), section: &str, args: &[(&str, &str)]) -> &'static str {
    let body = extract_section_raw(content, section);
    if body.is_empty() {
        return "";
    }

    let body_start = content.find(body).unwrap_or(0);
    let line_offset = content[..body_start].chars().filter(|&c| c == '\n').count() + 1;

    let parsed = if args.is_empty() { body.to_string() } else { parse_lua(body, args, path, path, 0) };

    Box::leak(format!("--@src:{path}:{line_offset}\n{parsed}").into_boxed_str())
}

pub fn extract_section_raw<'a>(content: &'a str, section: &str) -> &'a str {
    let start_marker = format!("--@{section}");
    let end_marker = "--@end";

    let Some(start_pos) = content.find(&start_marker) else {
        return "";
    };
    let after = start_pos + start_marker.len();
    let start = content[after..].find('\n').map(|nl| after + nl + 1).unwrap_or(after);
    let rest = &content[start..];
    let end = rest.find(end_marker).map(|i| start + i).unwrap_or(content.len());
    content[start..end].trim()
}
