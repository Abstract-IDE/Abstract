//! Progress indication: a determinate [`ProgressBar`] and an animated
//! [`Spinner`] driven by a timer-owning [`SpinnerState`].

use std::rc::Rc;

use crate::error::Result;
use crate::nvim::Timer;
use crate::reactive::Signal;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Size, Widget};

/// A determinate progress bar. Rebuild with a fresh fraction each frame — the
/// value that drives it usually comes from a `Signal` read in the build closure.
pub struct ProgressBar {
    fraction: f32,
    label: Option<String>,
    filled: char,
    empty: char,
}

impl ProgressBar {
    pub fn new(fraction: f32) -> Self {
        Self { fraction: fraction.clamp(0.0, 1.0), label: None, filled: '█', empty: '░' }
    }
    /// Overlay a centered label (e.g. `"42%"`).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn chars(mut self, filled: char, empty: char) -> Self {
        self.filled = filled;
        self.empty = empty;
        self
    }
}

impl Widget for ProgressBar {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, 1)
    }

    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let filled = ((area.w as f32) * self.fraction).round() as u16;
        for x in 0..area.w {
            let (ch, hl) = if x < filled {
                (self.filled, groups::ACCENT)
            } else {
                (self.empty, groups::MUTED)
            };
            canvas.set(area.x + x, area.y, ch, Some(Rc::from(hl)));
        }
        if let Some(label) = &self.label {
            let lw = crate::text::display_width(label);
            if lw <= area.w {
                let x = area.x + (area.w - lw) / 2;
                canvas.put_str(x, area.y, label, Some(Rc::from(groups::TITLE)));
            }
        }
    }
}

/// Owns the timer + frame signal behind a [`Spinner`]. Create it ONCE (outside
/// the build closure); every widget reading its signal repaints per tick, and
/// dropping the state stops the animation.
#[derive(Clone)]
pub struct SpinnerState {
    frame: Signal<usize>,
    _timer: Rc<Timer>,
}

impl SpinnerState {
    /// Start ticking every `interval_ms`.
    pub fn start(interval_ms: u64) -> Result<Self> {
        let frame = Signal::new(0usize);
        let sig = frame.clone();
        let timer = Timer::interval(interval_ms.max(16), move || {
            sig.update(|f| *f = f.wrapping_add(1));
        })?;
        Ok(Self { frame, _timer: Rc::new(timer) })
    }

    /// Stop the animation (also happens when the last clone is dropped).
    pub fn stop(&self) {
        let _ = self._timer.close();
    }

    /// The reactive frame counter.
    pub fn frame(&self) -> Signal<usize> {
        self.frame.clone()
    }
}

/// Default spinner frames (braille).
pub const SPINNER_DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// An animated spinner: paints the frame the [`SpinnerState`] is on.
pub struct Spinner {
    state: SpinnerState,
    frames: &'static [&'static str],
    hl: &'static str,
}

impl Spinner {
    pub fn new(state: &SpinnerState) -> Self {
        Self { state: state.clone(), frames: SPINNER_DOTS, hl: groups::ACCENT }
    }
    pub fn frames(mut self, frames: &'static [&'static str]) -> Self {
        self.frames = frames;
        self
    }
    pub fn fg(mut self, group: &'static str) -> Self {
        self.hl = group;
        self
    }
}

impl Widget for Spinner {
    fn measure(&self, c: Constraints) -> Size {
        let w = self.frames.iter().map(|f| crate::text::display_width(f)).max().unwrap_or(1);
        Size::new(w.min(c.max_w), 1)
    }

    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        if self.frames.is_empty() {
            return;
        }
        let i = self.state.frame.get() % self.frames.len(); // tracked: repaint per tick
        canvas.put_str(area.x, area.y, self.frames[i], Some(Rc::from(self.hl)));
    }
}
