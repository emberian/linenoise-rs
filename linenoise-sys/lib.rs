extern crate libc;

use libc::{c_char, c_int, size_t};

#[repr(C)]
pub struct Completions {
    len: size_t,
    cvec: *mut *mut c_char,
}

pub type CompletionCallback = extern "C" fn(*const c_char,
                                            *mut Completions);

extern "C" {
    pub fn linenoiseSetCompletionCallback(cb: Option<CompletionCallback>);
    pub fn linenoiseAddCompletion(completions: *mut Completions, string: *const c_char);

    pub fn linenoise(prompt: *const c_char) -> *mut c_char;
    pub fn linenoiseHistoryAdd(line: *const c_char) -> c_int;
    pub fn linenoiseHistorySetMaxLen(len: c_int) -> c_int;
    pub fn linenoiseHistorySave(filename: *const c_char) -> c_int;
    pub fn linenoiseHistoryLoad(filename: *const c_char) -> c_int;
    pub fn linenoiseClearScreen();
    pub fn linenoiseSetMultiLine(ml: c_int);
    pub fn linenoisePrintKeyCodes();
}
