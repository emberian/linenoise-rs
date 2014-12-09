//! Simple [linenoise](https://github.com/antirez/linenoise/) wrapper.
//!
//! Since linenoise is not thread safe, all of these functions internally acquire a global mutex
//! before calling the method. This is somewhat unfortunate.

extern crate "linenoise-sys" as linenoise;

use std::c_str::CString;

static LOCK: std::sync::StaticMutex = std::sync::MUTEX_INIT;

/// Prompt for input with string `p`.
pub fn prompt(p: &str) -> CString {
    let _lock = LOCK.lock();
    unsafe {
        CString::new(linenoise::linenoise(p.as_ptr() as *const _) as *const _, true)
    }
}
