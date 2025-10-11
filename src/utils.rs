use std::io::{self, Write};

pub fn flush_stdout() {
    if let Err(e) = io::stdout().flush() {
        eprintln!("Error flush stdout: {}", e);
    }
}