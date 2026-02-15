// ── Plugin Spec System ──
//
// Two ways to define a plugin:
//
//   lua_spec!(r#"{ "author/plugin", lazy = true }"#)
//
//   lua_spec!(lua_file!("plugin.lua"), &[("VAR", "value")])
//
// Both return `SpecInfo`. Validation in lazy.rs handles error
// reporting for both — inline specs map to Rust lines,
// file specs resolve `--@src:` markers to the actual .lua file.

pub struct SpecInfo {
    pub spec: &'static str,
    pub file: &'static str,
    pub spec_line: u32,
}

#[macro_export]
macro_rules! lua_spec {
    // Raw Lua string (inline in .rs file)
    ($spec:expr) => {
        $crate::plugins::spec::SpecInfo { spec: $spec, file: file!(), spec_line: line!() }
    };
    // Lua file with template args
    ($file:expr, $args:expr) => {{
        let (path, content) = $file;
        let parsed = $crate::plugins::spec::parse_lua(content, $args, path, file!(), line!());
        let tagged = format!("(function()\n--@src:{path}\n{parsed}\nend)()");
        $crate::plugins::spec::SpecInfo { spec: Box::leak(tagged.into_boxed_str()), file: file!(), spec_line: line!() }
    }};
    // Lua file, no args
    ($file:expr,_) => {
        lua_spec!($file, &[] as &[(&str, &str)])
    };
}

/// Pairs a relative path with `include_str!` for use with `lua_spec!`.
#[macro_export]
macro_rules! lua_file {
    ($path:literal) => {
        ($path, include_str!($path))
    };
}

// ── Template Parser ──
//
// Replaces `--[[@rs $VAR ]]` markers in Lua files with values from Rust.
// Panics at startup if a `$VAR` has no matching arg.
pub fn parse_lua(content: &str, args: &[(&str, &str)], lua_path: &str, rust_file: &str, rust_line: u32) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;
    let arg_names: Vec<&str> = args.iter().map(|(n, _)| *n).collect();

    while let Some(start) = rest.find("--[[@rs") {
        result.push_str(&rest[..start]);

        let after_marker = &rest[start + 7..];
        if let Some(end) = after_marker.find("]]") {
            let template = after_marker[..end].trim();
            check_unresolved(template, &arg_names, lua_path, rust_file, rust_line);

            let mut expanded = template.to_string();
            for (name, val) in args {
                expanded = expanded.replace(&format!("${name}"), val);
            }

            result.push_str(&expanded);
            rest = &after_marker[end + 2..];
        } else {
            result.push_str(&rest[start..start + 7]);
            rest = &rest[start + 7..];
        }
    }

    result.push_str(rest);
    result
}

fn check_unresolved(template: &str, arg_names: &[&str], lua_path: &str, rust_file: &str, rust_line: u32) {
    let mut scan = template;
    while let Some(pos) = scan.find('$') {
        let var_end = scan[pos + 1..].find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(scan.len() - pos - 1);
        let var_name = &scan[pos + 1..pos + 1 + var_end];
        if !var_name.is_empty() && !arg_names.contains(&var_name) {
            panic!(
                "\n[Abstract] Unresolved variable `${var_name}`\n  \
                 --> template: {lua_path}\n  \
                 --> called from: {rust_file}:{rust_line}\n  \
                 Hint: pass (\"{var_name}\", \"value\") in the args\n",
            );
        }
        scan = &scan[pos + 1 + var_end..];
    }
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
        if let Some(rest) = line.trim().strip_prefix("--@src:") {
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
pub fn extract_section<'a>(content: &'a str, section: &str) -> &'a str {
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

/// Same as `extract_section` but injects `--@src:path:offset` for error tracing.
pub fn extract_section_tracked((path, content): (&str, &str), section: &str) -> &'static str {
    let body = extract_section(content, section);
    if body.is_empty() {
        return "";
    }

    let body_start = content.find(body).unwrap_or(0);
    let line_offset = content[..body_start].chars().filter(|&c| c == '\n').count() + 1;

    Box::leak(format!("--@src:{path}:{line_offset}\n{body}").into_boxed_str())
}
