use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

use super::{
    error::IndentError,
    types::{IndentConfig, SampleMode},
};

/// Result of scanning a file or buffer, containing the extracted bytes
/// and a flag indicating if the content appears to be binary.
pub struct ScanResult {
    pub content: Vec<u8>,
    pub is_binary: bool,
}

/// Handles reading and sampling data from files or memory buffers,
/// applying binary detection heuristics before full processing.
pub struct Scanner {
    config: IndentConfig,
}

impl Scanner {
    pub fn new(config: IndentConfig) -> Self {
        Scanner { config }
    }

    /// Scans a file from disk. Attempts memory mapping for performance
    /// if a full read is requested, falling back to buffered reading.
    pub fn scan_file(&self, path: &Path) -> Result<ScanResult, IndentError> {
        let file = File::open(path).map_err(IndentError::Io)?;
        let file_size = file.metadata().map_err(IndentError::Io)?.len();

        // Fast-path: Memory map full files instead of copying bytes into memory
        if let SampleMode::Full = self.config.sample_mode
            && file_size > 0
        {
            // SAFETY: We assume the underlying file is not truncated or modified
            // by another process while the map is active.
            if let Ok(mmap) = unsafe { memmap2::Mmap::map(&file) } {
                return self.process_raw_bytes(&mmap, file_size);
            }
        }

        let mut reader = BufReader::new(file);
        self.scan_reader(&mut reader, file_size)
    }

    /// Scans an in-memory byte slice (e.g., a live Neovim buffer).
    pub fn scan_bytes(&self, bytes: &[u8]) -> Result<ScanResult, IndentError> {
        let size = bytes.len() as u64;

        if let SampleMode::Full = self.config.sample_mode {
            return self.process_raw_bytes(bytes, size);
        }

        // Wrap the slice in a Cursor so it implements Read + Seek,
        // allowing us to reuse the exact same logic as file reading.
        let mut reader = Cursor::new(bytes);
        self.scan_reader(&mut reader, size)
    }

    /// The core reading engine. Handles binary peeking and configured sampling strategies.
    fn scan_reader<R: BufRead + Seek>(&self, reader: &mut R, total_size: u64) -> Result<ScanResult, IndentError> {
        // 1. Peek at the first 1KB to run binary heuristics before doing heavy lifting
        let mut buf = [0u8; 1024];
        let n = reader.read(&mut buf).map_err(IndentError::Io)?;
        reader.seek(SeekFrom::Start(0)).map_err(IndentError::Io)?;

        if self.is_buffer_binary(&buf[..n], total_size) {
            return Ok(ScanResult { content: Vec::new(), is_binary: true });
        }

        // 2. Extract content based on the requested sampling mode
        let content = match self.config.sample_mode {
            SampleMode::Full => {
                let mut buffer = Vec::with_capacity(total_size as usize);
                reader.read_to_end(&mut buffer).map_err(IndentError::Io)?;
                buffer
            },
            SampleMode::Lines(num_lines) => {
                let mut buffer = Vec::new();
                let mut line = String::new();
                let mut lines_read = 0;

                while lines_read < num_lines {
                    line.clear();
                    let bytes = reader.read_line(&mut line).map_err(IndentError::Io)?;
                    if bytes == 0 {
                        break;
                    }

                    lines_read += 1;
                    buffer.extend_from_slice(line.as_bytes());
                }
                buffer
            },
            SampleMode::Percent(percent) => {
                let bytes_to_read = ((total_size * percent.min(100) as u64) / 100).max(1).min(total_size);
                let mut buffer = Vec::with_capacity(bytes_to_read as usize);

                // Use by_ref() to avoid consuming the original reader wrapper
                let mut chunk_reader = reader.by_ref().take(bytes_to_read);
                chunk_reader.read_to_end(&mut buffer).map_err(IndentError::Io)?;

                buffer
            },
        };

        Ok(ScanResult { content, is_binary: false })
    }

    /// Fast-path helper for mmap and fully-loaded in-memory buffers.
    fn process_raw_bytes(&self, bytes: &[u8], total_size: u64) -> Result<ScanResult, IndentError> {
        let peek_size = (total_size as usize).min(1024);

        if self.is_buffer_binary(&bytes[..peek_size], total_size) {
            Ok(ScanResult { content: Vec::new(), is_binary: true })
        } else {
            Ok(ScanResult { content: bytes.to_vec(), is_binary: false })
        }
    }

    /// Uses multiple heuristics to determine if a byte slice is likely binary data.
    fn is_buffer_binary(&self, buffer: &[u8], total_size: u64) -> bool {
        // Absolute indicator: text files rarely contain null bytes
        if buffer.contains(&0) {
            return true;
        }
        if total_size == 0 {
            return false;
        }

        let len = buffer.len() as f64;
        if len == 0.0 {
            return false;
        }

        // Calculate Shannon entropy to detect highly randomized/compressed data
        let mut freq = [0usize; 256];
        for &b in buffer {
            freq[b as usize] += 1;
        }

        let mut entropy = 0.0;
        for &count in freq.iter() {
            if count == 0 {
                continue;
            }
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }

        // Entropy > 7.5 bits per byte strongly suggests binary/compressed data
        if entropy > 7.5 {
            return true;
        }

        // Check ratio of printable ASCII characters + common whitespace
        let printable_count =
            buffer.iter().filter(|&&b| b == 0x09 || b == 0x0A || b == 0x0D || (0x20..0x7F).contains(&b)).count();

        // If less than 30% of the buffer is standard text, assume it's binary
        if (printable_count as f64) < 0.3 {
            return true;
        }

        false
    }
}
