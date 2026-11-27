//! File icons, in each file type's own colour, from the user's icon plugin —
//! `mini.icons` when it is set up, else `nvim-web-devicons` — cached per file
//! name. Without either plugin there are no icons (`None`), so callers just
//! leave the icon out.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::lua;

thread_local! {
    static CACHE: RefCell<HashMap<String, Option<(String, String)>>> = RefCell::new(HashMap::new());
}

/// The icon and its highlight group for a file (by name or path).
pub fn file(name: &str) -> Option<(String, String)> {
    let key = name.rsplit(['/', '\\']).next().unwrap_or(name);
    if let Some(hit) = CACHE.with(|c| c.borrow().get(key).cloned()) {
        return hit;
    }
    let Ok((icon, hl, fin)) =
        lua::driver_fn("file_icon").and_then(|f| Ok(f.call::<(Option<String>, Option<String>, bool)>(key)?))
    else {
        return None;
    };
    let found = icon.filter(|i| !i.is_empty()).map(|i| (i, hl.unwrap_or_default()));
    // Only the final provider's answers are kept: a plugin loaded later
    // (or mini.icons once set up) may still give this file an icon/colour.
    if fin {
        CACHE.with(|c| c.borrow_mut().insert(key.to_string(), found.clone()));
    }
    found
}
