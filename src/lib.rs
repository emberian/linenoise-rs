#![allow(unstable)]

//! Simple [linenoise](https://github.com/antirez/linenoise/) wrapper.
//!
//! Since linenoise is not thread safe, all of these functions internally acquire a global mutex
//! before calling the method. This is somewhat unfortunate.

extern crate "linenoise-sys" as linenoise;
extern crate libc;

use std::ffi::CString;

fn from_c_str<'a>(p: &'a *const libc::c_char) -> &'a str {
    std::str::from_utf8( unsafe { std::ffi::c_str_to_bytes(p) } ).ok().expect("Found invalid utf8")
}

/// Prompt for input with string `p`. Returns `None` when there was no input, `Some` otherwise.
pub fn prompt(p: &str) -> Option<String> {
    unsafe {
        let ptr = CString::from_slice(p.as_bytes()).as_slice_with_nul().as_ptr();
        let res = linenoise::linenoise(ptr);
        if res.is_null() {
            None
        } else {
            let cr = res as *const _;
            Some(from_c_str(&cr).to_string())
        }
    }
}

pub type CompletionCallback = fn(&str) -> Vec<&str>;
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
        let input = from_c_str(&cr);
        for external_callback in USER_COMPLETION.iter() {
            let ret = (*external_callback)(input);
            for x in ret.iter() {
                add_completion(lc, *x);
            }
        }
    }
}

/// Add a completion to the current list of completions.
pub fn add_completion(c: *mut linenoise::Completions, s: &str) {
    unsafe {
        linenoise::linenoiseAddCompletion(c, CString::from_slice(s.as_bytes()).as_slice_with_nul().as_ptr());
    }
}


/// Add this string to the history
pub fn history_add(line: &str) -> i32 {
    let cs = CString::from_slice(line.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = linenoise::linenoiseHistoryAdd(cs);
    }
    ret
}

/// Set max length history
pub fn history_set_max_len(len: i32) -> i32 {
    let mut ret: i32;
    unsafe {
        ret = linenoise::linenoiseHistorySetMaxLen(len);
    }
    ret
}

/// Save the history on disk
pub fn history_save(file: &str) -> i32 {
    let fname = CString::from_slice(file.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = linenoise::linenoiseHistorySave(fname);
    }
    ret
}

/// Load the history on disk
pub fn history_load(file: &str) -> i32 {
    let fname = CString::from_slice(file.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = linenoise::linenoiseHistoryLoad(fname);
    }
    ret
}

///Clears the screen
pub fn clear_screen() {
    unsafe {
        linenoise::linenoiseClearScreen();
    }
}

pub fn set_multiline(ml: i32) {
    unsafe {
        linenoise::linenoiseSetMultiLine(ml);
    }
}

pub fn print_key_codes() {
    unsafe {
        linenoise::linenoisePrintKeyCodes();
    }
}
