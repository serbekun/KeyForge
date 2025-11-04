use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref BREAK_FLAG: Mutex<bool> = Mutex::new(false);
    static ref CONTINUE_FLAG: Mutex<bool> = Mutex::new(false);
}

pub fn set_break_flag(value: bool) {
    let mut flag = BREAK_FLAG.lock().unwrap();
    *flag = value;
}

pub fn set_continue_flag(value: bool) {
    let mut flag = CONTINUE_FLAG.lock().unwrap();
    *flag = value;
}

pub fn should_break() -> bool {
    let flag = BREAK_FLAG.lock().unwrap();
    *flag
}

pub fn should_continue() -> bool {
    let flag = CONTINUE_FLAG.lock().unwrap();
    *flag
}

pub fn reset_loop_flags() {
    set_break_flag(false);
    set_continue_flag(false);
}