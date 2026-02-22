# Indentation Detection Library

# Features

- **Indentation Style Detection:** Distinguishes between space indentation (and the typical indent width), tab indentation, mixed indentation, no indentation, or binary content.
- **Memory Mapping:** Uses `memmap2` to efficiently memory-map files for fast reading of large files (with a fallback to normal I/O if mapping is not possible).
- **Parallel Analysis:** Utilizes `rayon` to speed up analysis of large files by parallelizing line inspection.
- **Robust Encoding Handling:** Gracefully handles UTF-8 files and falls back to byte-based analysis for files with invalid UTF-8 or other encodings (including detection of UTF-16 via BOM).
- **Binary Detection:** Defensively detects binary or non-text files using null-byte presence and entropy analysis to avoid misclassifying binary content as text:contentReference[oaicite:0]{index=0}.
- **Configurable Sampling:** Supports analyzing the entire file or only a sample (first N lines or first N% of the file) for performance.

# How it Works

The library reads the file (via memory map or buffered I/O) and inspects the beginning of each line to count leading whitespace characters.
It determines if the indentation is consistently spaces or tabs, or if both are used.
It also treats files with no indentation as `IndentStyle::None`, and uses heuristics (null bytes and Shannon entropy) to mark a file as `IndentStyle::Binary` if it's not likely text:contentReference[oaicite:1]{index=1}.
For files with invalid UTF-8 sequences, it will fall back to analyzing the raw bytes to still detect indentation correctly.
