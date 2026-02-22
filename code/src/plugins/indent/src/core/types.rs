use std::{
    default::Default,
    fmt, //
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndentStyle {
    /// Indentation uses tabs exclusively.
    Tab,
    /// Indentation uses spaces exclusively.
    /// The associated value is an estimate of spaces per indent level (e.g., 4 or 2 spaces).
    Space(usize),
    /// Mixed indentation (some lines use tabs, others use spaces, or tabs and spaces mixed in the same line).
    Mixed,
    /// No indentation detected (e.g., all lines start at column 0, or the file is empty).
    None,
    /// The file is binary or not recognized as text, so indentation style is not applicable.
    Binary,
}

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IndentStyle::Tab => write!(f, "Tabs"),
            IndentStyle::Space(n) => {
                if n > 1 {
                    write!(f, "Spaces ({} spaces per indent)", n)
                } else {
                    write!(f, "Spaces")
                }
            },
            IndentStyle::Mixed => write!(f, "Mixed"),
            IndentStyle::None => write!(f, "None"),
            IndentStyle::Binary => write!(f, "Binary"),
        }
    }
}

/// Specifies how much of the file to analyze.
#[derive(Debug, Clone)]
pub enum SampleMode {
    /// Analyze the entire file.
    Full,
    /// Analyze only the first N lines of the file.
    Lines(usize),
    /// Analyze only the first N percent of the file (1-100).
    Percent(u8),
}

/// Configuration options for indentation detection.
#[derive(Debug, Clone)]
pub struct IndentConfig {
    /// Sampling mode for reading the file (full vs partial).
    pub sample_mode: SampleMode,
}

impl IndentConfig {
    /// Builder-style method to set the sampling mode.
    pub fn with_sample_mode(mut self, mode: SampleMode) -> Self {
        self.sample_mode = mode;
        self
    }
}

impl Default for IndentConfig {
    /// Create a default configuration that analyzes the full file.
    fn default() -> Self {
        Self { sample_mode: SampleMode::Full }
    }
}
