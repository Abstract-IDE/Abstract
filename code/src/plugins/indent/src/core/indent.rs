use std::path::Path;

pub use super::{
    analyzer::IndentAnalyzer,
    error::IndentError,
    scanner::{ScanResult, Scanner},
    types::{IndentConfig, IndentStyle},
};

/// Main public interface for indentation detection.
pub struct Indent {
    config: IndentConfig,
}

#[allow(unused)]
impl Indent {
    pub fn new(config: IndentConfig) -> Self {
        Self { config }
    }

    /// Detect indentation by reading directly from a file path on disk.
    pub fn detect_from_file<P: AsRef<Path>>(&self, path: P) -> Result<IndentStyle, IndentError> {
        detect_indent_file(path.as_ref(), &self.config)
    }

    /// Detect indentation from an existing in-memory byte slice (e.g. a Neovim buffer).
    pub fn detect_from_bytes(&self, bytes: &[u8]) -> Result<IndentStyle, IndentError> {
        detect_indent_bytes(bytes, &self.config)
    }
}

/// Standalone function to scan and analyze a file.
pub fn detect_indent_file<P: AsRef<Path>>(path: P, config: &IndentConfig) -> Result<IndentStyle, IndentError> {
    let scanner = Scanner::new(config.clone());
    let scan_result = scanner.scan_file(path.as_ref())?;
    analyze_scan_result(scan_result)
}

/// Standalone function to scan and analyze raw in-memory bytes.
pub fn detect_indent_bytes(bytes: &[u8], config: &IndentConfig) -> Result<IndentStyle, IndentError> {
    let scanner = Scanner::new(config.clone());
    let scan_result = scanner.scan_bytes(bytes)?;
    analyze_scan_result(scan_result)
}

/// Routes the output of the Scanner to the IndentAnalyzer.
fn analyze_scan_result(scan_result: ScanResult) -> Result<IndentStyle, IndentError> {
    if scan_result.is_binary {
        return Ok(IndentStyle::Binary);
    }

    let style = IndentAnalyzer::new().analyze(&scan_result.content);
    Ok(style)
}
