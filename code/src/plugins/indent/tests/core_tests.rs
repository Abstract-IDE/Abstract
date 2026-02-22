mod utils;

use abstract_indent::core::{
    indent::Indent,
    types::{IndentConfig, IndentStyle, SampleMode},
};

use crate::utils::write_temp_file;

#[test]
fn test_spaces_indentation() {
    let content = b"hello\n    world\n    foo\n";
    let path = write_temp_file("test_spaces.txt", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).expect("failed to detect indent from file");
    let style_bytes = indent.detect_from_bytes(content).expect("failed to detect indent from bytes");

    assert!(matches!(style_file, IndentStyle::Space(n) if n == 4));
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_tabs_indentation() {
    let content = b"line1\n\tline2\n\t\tline3\n";
    let path = write_temp_file("test_tabs.txt", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(content).unwrap();

    assert_eq!(style_file, IndentStyle::Tab);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_mixed_indentation() {
    let content = b"def\n\tif True:\n    return 42\n";
    let path = write_temp_file("test_mixed.txt", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(content).unwrap();

    assert_eq!(style_file, IndentStyle::Mixed);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_none_indentation() {
    let content = b"foo\nbar\nbaz\n";
    let path = write_temp_file("test_none.txt", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(content).unwrap();

    assert_eq!(style_file, IndentStyle::None);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_crlf_handling() {
    let content = b"first\r\n    second\r\n\tthird\r\n";
    let path = write_temp_file("test_crlf.txt", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(content).unwrap();

    assert_eq!(style_file, IndentStyle::Mixed);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_binary_detection_null_bytes() {
    let content = b"\xFF\x00\xFF\x00";
    let path = write_temp_file("test_bin_null.dat", content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(content).unwrap();

    assert_eq!(style_file, IndentStyle::Binary);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_binary_detection_entropy() {
    let mut content = Vec::with_capacity(256);
    for i in 0..256u16 {
        content.push((i % 256) as u8);
    }
    let path = write_temp_file("test_bin_entropy.dat", &content);

    let indent = Indent::new(IndentConfig::default());
    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(&content).unwrap();

    assert_eq!(style_file, IndentStyle::Binary);
    assert_eq!(style_file, style_bytes);
}

#[test]
fn test_sampling_first_lines() {
    let content = b"noindent\n    indent_space\n\tindent_tab\n";
    let path = write_temp_file("test_sample_lines.txt", content);

    // Test 1 line sample
    let config_1 = IndentConfig::default().with_sample_mode(SampleMode::Lines(1));
    let indent_1 = Indent::new(config_1);

    assert_eq!(indent_1.detect_from_file(&path).unwrap(), IndentStyle::None);
    assert_eq!(indent_1.detect_from_bytes(content).unwrap(), IndentStyle::None);

    // Test 2 line sample
    let config_2 = IndentConfig::default().with_sample_mode(SampleMode::Lines(2));
    let indent_2 = Indent::new(config_2);

    let style2_file = indent_2.detect_from_file(&path).unwrap();
    let style2_bytes = indent_2.detect_from_bytes(content).unwrap();

    assert!(matches!(style2_file, IndentStyle::Space(_)));
    assert_eq!(style2_file, style2_bytes);
}

#[test]
fn test_sampling_first_percent() {
    let mut content = Vec::new();
    content.extend_from_slice(b"    spaced\n");
    for _ in 0..1000 {
        content.extend_from_slice(b"x\n");
    }
    content.extend_from_slice(b"\tTabbed\n");

    let path = write_temp_file("test_sample_percent.txt", &content);

    let config = IndentConfig::default().with_sample_mode(SampleMode::Percent(50));
    let indent = Indent::new(config);

    let style_file = indent.detect_from_file(&path).unwrap();
    let style_bytes = indent.detect_from_bytes(&content).unwrap();

    assert!(matches!(style_file, IndentStyle::Space(_)));
    assert_ne!(style_file, IndentStyle::Mixed);
    assert_eq!(style_file, style_bytes);
}
