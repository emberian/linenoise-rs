//! Simple [linenoise](https://github.com/antirez/linenoise/) wrapper.
//!
//! Since linenoise is not thread safe, all of these functions internally acquire a global mutex
//! before calling the method. This is somewhat unfortunate.

extern crate "linenoise-sys" as linenoise;

use std::c_str::CString;

static LOCK: std::sync::StaticMutex = std::sync::MUTEX_INIT;

/// Prompt for input with string `p`. Returns `None` when there was no input, `Some` otherwise.
pub fn prompt(p: &str) -> Option<CString> {
    use std::c_str::ToCStr;

    let _lock = LOCK.lock();
    unsafe {
        let res = p.with_c_str(|ptr| linenoise::linenoise(ptr));
        if res.is_null() {
            None
        } else {
            Some(CString::new(res as *const _, true))
        }
    }
}
