//! Simple [linenoise](https://github.com/antirez/linenoise/) wrapper.
//!
//! Since linenoise is not thread safe, all of these functions internally acquire a global mutex
//! before calling the method. This is somewhat unfortunate.

extern crate libc;
extern crate linenoise_sys as linenoise;

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

fn from_c_str(p: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(p) };
    String::from(std::str::from_utf8(c_str.to_bytes()).unwrap())
}

/// Prompt for input with string `p`. Returns `None` when there was no input, `Some` otherwise.
pub fn prompt(p: &str) -> Option<String> {
    let p_utf8: Vec<u8> = p.bytes().collect();
    let prompt = CString::new(p_utf8).unwrap();
    let res = unsafe { linenoise::linenoise(prompt.as_bytes_with_nul().as_ptr() as *const i8) };
    if res.is_null() {
        None
    } else {
        let cr = res as *const _;
        Some(from_c_str(cr).to_string())
    }
}

pub type CompletionCallback = fn(&str) -> Vec<String>;
static mut USER_COMPLETION: Option<CompletionCallback> = None;

/// Sets the callback when tab is pressed
pub fn set_callback(rust_cb: CompletionCallback ) {
    unsafe {
        USER_COMPLETION = Some(rust_cb);
        let ca = internal_callback as *mut _;
        linenoise::linenoiseSetCompletionCallback(ca);
    }
}

fn internal_callback(cs: *mut libc::c_char, lc:*mut linenoise::Completions ) {
    unsafe {
        (*lc).len = 0;
        let cr = cs as *const _;
        let input = from_c_str(cr);
        for external_callback in USER_COMPLETION.iter() {
            let ret = (*external_callback)(&input);
            for x in ret.iter() {
                add_completion(lc, x);
            }
        }
    }
}

/// Add a completion to the current list of completions.
pub fn add_completion(c: *mut linenoise::Completions, s: &str) {
    unsafe {
        let c_str = CString::new(s).unwrap();
        linenoise::linenoiseAddCompletion(c, c_str.to_bytes_with_nul().as_ptr() as *const i8);
    }
}


/// Add this string to the history
pub fn history_add(line: &str) -> i32 {
    let c_str = CString::new(line).unwrap();
    unsafe {
        linenoise::linenoiseHistoryAdd(c_str.to_bytes_with_nul().as_ptr() as *const i8)
    }
}

/// Set max length history
pub fn history_set_max_len(len: c_int) -> c_int {
    unsafe {
        linenoise::linenoiseHistorySetMaxLen(len)
    }
}

/// Save the history on disk
pub fn history_save(file: &str) -> c_int {
    let c_str = CString::new(file).unwrap();
    let fname = c_str.to_bytes_with_nul().as_ptr() as *const i8;
    unsafe {
        linenoise::linenoiseHistorySave(fname)
    }
}

/// Load the history on disk
pub fn history_load(file: &str) -> c_int {
    let c_str = CString::new(file).unwrap();
    let fname = c_str.to_bytes_with_nul().as_ptr() as *const i8;
    unsafe {
        linenoise::linenoiseHistoryLoad(fname)
    }
}

///Clears the screen
pub fn clear_screen() {
    unsafe {
        linenoise::linenoiseClearScreen();
    }
}

pub fn set_multiline(ml: c_int) {
    unsafe {
        linenoise::linenoiseSetMultiLine(ml);
    }
}

pub fn print_key_codes() {
    unsafe {
        linenoise::linenoisePrintKeyCodes();
    }
}
