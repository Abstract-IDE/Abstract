//! Media in the terminal — images, video frames, PDF pages — with the kitty
//! graphics protocol and its unicode placeholders (kitty, Ghostty). No Neovim plugin involved:
//!
//! - images are decoded in Rust (png, jpg, gif, webp, bmp, tiff, ico); a video
//!   frame needs `ffmpeg`, a PDF page `pdftoppm` or `mutool` (used when
//!   installed);
//! - the picture is sent to the terminal as PNG (`nvim_ui_send`), then shown
//!   through *Unicode placeholders*: cells of `U+10EEEE` in a buffer, whose
//!   colour carries the image id. So the image lives in buffer text — it moves,
//!   clips and scrolls with the window like any text.
//!
//! [`CodeView::show_media`](crate::view::CodeView::show_media) uses this.

use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::error::Result;
use crate::lua;

const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp", "tif", "tiff", "ico"];
const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "mov", "webm", "avi", "m4v"];

/// The longest side a picture is sent with (bigger ones are scaled down).
const MAX_PIXELS: u32 = 1600;

fn ext(path: &str) -> String {
    Path::new(path).extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default()
}

fn on_path(program: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join(program).is_file()))
}

/// Whether the terminal can show images: a kitty-graphics terminal, and
/// `termguicolors` (the image id travels in the text colour).
pub fn terminal_supported() -> bool {
    let env = |k: &str| std::env::var(k).unwrap_or_default();
    // Inside tmux, or a Neovim nested in a `:terminal`, the variables are
    // inherited from the outer kitty but the escapes never reach it.
    if !env("TMUX").is_empty() || !env("NVIM").is_empty() {
        return false;
    }
    // Unicode placeholders (U=1) are kitty's and Ghostty's; WezTerm speaks the
    // graphics protocol but not those.
    let kitty_like = !env("KITTY_WINDOW_ID").is_empty()
        || env("TERM").contains("kitty")
        || env("TERM").contains("ghostty")
        || !env("GHOSTTY_RESOURCES_DIR").is_empty()
        || env("TERM_PROGRAM") == "ghostty";
    kitty_like && lua::get_option::<bool>("termguicolors").unwrap_or(false)
}

/// Whether `path` is media this module can show (images always; video and
/// PDF when their converter is installed).
pub fn is_media(path: &str) -> bool {
    let e = ext(path);
    IMAGE_EXTS.contains(&e.as_str())
        || (VIDEO_EXTS.contains(&e.as_str()) && on_path("ffmpeg"))
        || (e == "pdf" && (on_path("pdftoppm") || on_path("mutool")))
}

/// A picture ready to send: PNG bytes and its size in pixels.
#[derive(Clone, Debug)]
pub struct Picture {
    pub png: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Decode `path` into a PNG picture (scaled down when large). Blocking — run
/// it off the main thread.
pub fn decode(path: &str) -> std::result::Result<Picture, String> {
    let e = ext(path);
    let img = if VIDEO_EXTS.contains(&e.as_str()) {
        let frame = |at: &str| {
            Command::new("ffmpeg")
                .args([
                    "-v",
                    "error",
                    "-ss",
                    at,
                    "-i",
                    path,
                    "-frames:v",
                    "1",
                    "-f",
                    "image2pipe",
                    "-vcodec",
                    "png",
                    "-",
                ])
                .output()
        };
        // A frame a second in (skips black intros); short clips: the first.
        let out = frame("1").ok().filter(|o| !o.stdout.is_empty()).or_else(|| frame("0").ok());
        let png = out.map(|o| o.stdout).filter(|b| !b.is_empty()).ok_or("ffmpeg produced no frame")?;
        image::load_from_memory(&png).map_err(|e| e.to_string())?
    } else if e == "pdf" {
        let out = if on_path("pdftoppm") {
            Command::new("pdftoppm")
                .args(["-png", "-f", "1", "-l", "1", "-r", "110", "-singlefile", path, "-"])
                .output()
        } else {
            Command::new("mutool").args(["draw", "-q", "-F", "png", "-r", "110", "-o", "-", path, "1"]).output()
        };
        let png = out.map_err(|e| e.to_string())?.stdout;
        if png.is_empty() {
            return Err("couldn't render the PDF".into());
        }
        image::load_from_memory(&png).map_err(|e| e.to_string())?
    } else {
        image::ImageReader::open(path)
            .map_err(|e| e.to_string())?
            .with_guessed_format()
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?
    };
    let img = if img.width().max(img.height()) > MAX_PIXELS { img.thumbnail(MAX_PIXELS, MAX_PIXELS) } else { img };
    let (width, height) = (img.width(), img.height());
    let mut png = Vec::new();
    img.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png).map_err(|e| e.to_string())?;
    Ok(Picture { png, width, height })
}

/// A terminal cell's size in pixels (width, height), from the terminal;
/// a common default when it won't say.
pub fn cell_size() -> (f64, f64) {
    #[cfg(unix)]
    unsafe {
        let fd = libc::open(c"/dev/tty".as_ptr(), libc::O_RDONLY | libc::O_NOCTTY);
        if fd >= 0 {
            let mut ws: libc::winsize = std::mem::zeroed();
            let ok = libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) == 0;
            libc::close(fd);
            if ok && ws.ws_col > 0 && ws.ws_row > 0 && ws.ws_xpixel > 0 && ws.ws_ypixel > 0 {
                return (ws.ws_xpixel as f64 / ws.ws_col as f64, ws.ws_ypixel as f64 / ws.ws_row as f64);
            }
        }
    }
    (9.0, 18.0)
}

/// Columns × rows that show a `width`×`height` px picture as large as fits
/// in `max_cols`×`max_rows` cells, keeping its aspect (never enlarged).
pub fn fit(width: u32, height: u32, max_cols: u16, max_rows: u16, cell: (f64, f64)) -> (u16, u16) {
    // A placeholder cell names its row and column with a diacritic: no more
    // than there are diacritics.
    let limit = DIACRITICS.len() as u16;
    let (max_cols, max_rows) = (max_cols.min(limit), max_rows.min(limit));
    let (w_cells, h_cells) = (width as f64 / cell.0, height as f64 / cell.1);
    let scale = (max_cols as f64 / w_cells).min(max_rows as f64 / h_cells).min(1.0);
    let cols = (w_cells * scale).round().clamp(1.0, max_cols.max(1) as f64) as u16;
    let rows = (h_cells * scale).round().clamp(1.0, max_rows.max(1) as f64) as u16;
    (cols, rows)
}

/// A fresh image id (24-bit: it is encoded as a text colour), distinct per
/// Neovim process so several instances in one terminal don't collide.
pub fn next_id() -> u32 {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    let n = NEXT.fetch_add(1, Ordering::Relaxed) % 0x1000;
    ((std::process::id() % 0xFFF) + 1) << 12 | n.max(1)
}

fn send(data: &str) -> Result<()> {
    lua::call_api::<()>("nvim_ui_send", (data,))
}

/// Send a picture to the terminal under `id` (not shown yet).
pub fn transmit(id: u32, picture: &Picture) -> Result<()> {
    let data = base64(&picture.png);
    let chunks: Vec<&str> = data.as_bytes().chunks(4096).map(|c| std::str::from_utf8(c).unwrap_or("")).collect();
    let mut out = String::with_capacity(data.len() + chunks.len() * 32);
    for (i, chunk) in chunks.iter().enumerate() {
        let more = if i + 1 < chunks.len() { 1 } else { 0 };
        if i == 0 {
            out.push_str(&format!("\x1b_Ga=t,f=100,t=d,i={id},q=2,m={more};{chunk}\x1b\\"));
        } else {
            out.push_str(&format!("\x1b_Gm={more};{chunk}\x1b\\"));
        }
    }
    send(&out)
}

/// Show image `id` through placeholders: a virtual placement of
/// `cols`×`rows` cells (placing it again resizes it).
pub fn place(id: u32, cols: u16, rows: u16) -> Result<()> {
    send(&format!("\x1b_Ga=p,U=1,i={id},p=1,c={cols},r={rows},q=2\x1b\\"))
}

/// Forget image `id` (its data and placements).
pub fn delete(id: u32) {
    let _ = send(&format!("\x1b_Ga=d,d=I,i={id},q=2\x1b\\"));
    crate::theme::forget(&format!("AbNuiMedia{id}"));
}

/// The text that shows a placed image: `rows` lines of `cols` placeholder
/// cells. Colour them with [`highlight`]`(id)`.
pub fn placeholder_lines(cols: u16, rows: u16) -> Vec<String> {
    let diacritic = |n: u16| char::from_u32(DIACRITICS[(n as usize).min(DIACRITICS.len() - 1)]).unwrap_or('\u{305}');
    (0..rows)
        .map(|r| (0..cols).map(|c| format!("\u{10EEEE}{}{}", diacritic(r), diacritic(c))).collect())
        .collect()
}

/// The highlight group whose text colour is image `id` (what the terminal
/// reads from placeholder cells).
pub fn highlight(id: u32) -> Result<String> {
    let name = format!("AbNuiMedia{id}");
    // Kept across `:colorscheme`, or the image would vanish.
    crate::theme::fixed_fg(&name, id as i64)?;
    Ok(name)
}

fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

/// Row/column numbers of placeholder cells, as combining diacritics (the
/// kitty graphics protocol's `rowcolumn-diacritics` table).
const DIACRITICS: &[u32] = &[
    0x0305, 0x030D, 0x030E, 0x0310, 0x0312, 0x033D, 0x033E, 0x033F, 0x0346, 0x034A, 0x034B, 0x034C, 0x0350, 0x0351,
    0x0352, 0x0357, 0x035B, 0x0363, 0x0364, 0x0365, 0x0366, 0x0367, 0x0368, 0x0369, 0x036A, 0x036B, 0x036C, 0x036D,
    0x036E, 0x036F, 0x0483, 0x0484, 0x0485, 0x0486, 0x0487, 0x0592, 0x0593, 0x0594, 0x0595, 0x0597, 0x0598, 0x0599,
    0x059C, 0x059D, 0x059E, 0x059F, 0x05A0, 0x05A1, 0x05A8, 0x05A9, 0x05AB, 0x05AC, 0x05AF, 0x05C4, 0x0610, 0x0611,
    0x0612, 0x0613, 0x0614, 0x0615, 0x0616, 0x0617, 0x0657, 0x0658, 0x0659, 0x065A, 0x065B, 0x065D, 0x065E, 0x06D6,
    0x06D7, 0x06D8, 0x06D9, 0x06DA, 0x06DB, 0x06DC, 0x06DF, 0x06E0, 0x06E1, 0x06E2, 0x06E4, 0x06E7, 0x06E8, 0x06EB,
    0x06EC, 0x0730, 0x0732, 0x0733, 0x0735, 0x0736, 0x073A, 0x073D, 0x073F, 0x0740, 0x0741, 0x0743, 0x0745, 0x0747,
    0x0749, 0x074A, 0x07EB, 0x07EC, 0x07ED, 0x07EE, 0x07EF, 0x07F0, 0x07F1, 0x07F3, 0x0816, 0x0817, 0x0818, 0x0819,
    0x081B, 0x081C, 0x081D, 0x081E, 0x081F, 0x0820, 0x0821, 0x0822, 0x0823, 0x0825, 0x0826, 0x0827, 0x0829, 0x082A,
    0x082B, 0x082C, 0x082D, 0x0951, 0x0953, 0x0954, 0x0F82, 0x0F83, 0x0F86, 0x0F87, 0x135D, 0x135E, 0x135F, 0x17DD,
    0x193A, 0x1A17, 0x1A75, 0x1A76, 0x1A77, 0x1A78, 0x1A79, 0x1A7A, 0x1A7B, 0x1A7C, 0x1B6B, 0x1B6D, 0x1B6E, 0x1B6F,
    0x1B70, 0x1B71, 0x1B72, 0x1B73, 0x1CD0, 0x1CD1, 0x1CD2, 0x1CDA, 0x1CDB, 0x1CE0, 0x1DC0, 0x1DC1, 0x1DC3, 0x1DC4,
    0x1DC5, 0x1DC6, 0x1DC7, 0x1DC8, 0x1DC9, 0x1DCB, 0x1DCC, 0x1DD1, 0x1DD2, 0x1DD3, 0x1DD4, 0x1DD5, 0x1DD6, 0x1DD7,
    0x1DD8, 0x1DD9, 0x1DDA, 0x1DDB, 0x1DDC, 0x1DDD, 0x1DDE, 0x1DDF, 0x1DE0, 0x1DE1, 0x1DE2, 0x1DE3, 0x1DE4, 0x1DE5,
    0x1DE6, 0x1DFE, 0x20D0, 0x20D1, 0x20D4, 0x20D5, 0x20D6, 0x20D7, 0x20DB, 0x20DC, 0x20E1, 0x20E7, 0x20E9, 0x20F0,
    0x2CEF, 0x2CF0, 0x2CF1, 0x2DE0, 0x2DE1, 0x2DE2, 0x2DE3, 0x2DE4, 0x2DE5, 0x2DE6, 0x2DE7, 0x2DE8, 0x2DE9, 0x2DEA,
    0x2DEB, 0x2DEC, 0x2DED, 0x2DEE, 0x2DEF, 0x2DF0, 0x2DF1, 0x2DF2, 0x2DF3, 0x2DF4, 0x2DF5, 0x2DF6, 0x2DF7, 0x2DF8,
    0x2DF9, 0x2DFA, 0x2DFB, 0x2DFC, 0x2DFD, 0x2DFE, 0x2DFF, 0xA66F, 0xA67C, 0xA67D, 0xA6F0, 0xA6F1, 0xA8E0, 0xA8E1,
    0xA8E2, 0xA8E3, 0xA8E4, 0xA8E5, 0xA8E6, 0xA8E7, 0xA8E8, 0xA8E9, 0xA8EA, 0xA8EB, 0xA8EC, 0xA8ED, 0xA8EE, 0xA8EF,
    0xA8F0, 0xA8F1, 0xAAB0, 0xAAB2, 0xAAB3, 0xAAB7, 0xAAB8, 0xAABE, 0xAABF, 0xAAC1, 0xFE20, 0xFE21, 0xFE22, 0xFE23,
    0xFE24, 0xFE25, 0xFE26, 0x10A0F, 0x10A38, 0x1D185, 0x1D186, 0x1D187, 0x1D188, 0x1D189, 0x1D1AA, 0x1D1AB, 0x1D1AC,
    0x1D1AD, 0x1D242, 0x1D243, 0x1D244,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_the_standard() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn fit_keeps_the_aspect_and_never_enlarges() {
        // 900×450 px at 9×18 px cells = 100×25 cells; into 50×50 → 50×13.
        assert_eq!(fit(900, 450, 50, 50, (9.0, 18.0)), (50, 13));
        // Tall: 90×900 px = 10×50 cells; into 50×20 → 4×20.
        assert_eq!(fit(90, 900, 50, 20, (9.0, 18.0)), (4, 20));
        // Small pictures keep their size.
        assert_eq!(fit(18, 18, 50, 50, (9.0, 18.0)), (2, 1));
    }

    #[test]
    fn placeholders_encode_row_and_column() {
        let lines = placeholder_lines(2, 2);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1], "\u{10EEEE}\u{030D}\u{0305}\u{10EEEE}\u{030D}\u{030D}");
        assert!(next_id() < 1 << 24);
        assert!(is_media("a/b.PNG") && !is_media("main.rs"));
    }

    #[test]
    fn decode_scales_large_pictures_down() {
        let dir = std::env::temp_dir().join(format!("ab-nui-media-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("big.png");
        image::RgbImage::new(3200, 800).save(&path).unwrap();
        let p = decode(&path.to_string_lossy()).unwrap();
        assert_eq!((p.width, p.height), (1600, 400));
        assert_eq!(&p.png[1..4], b"PNG");
        assert!(decode("/no/such.png").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
