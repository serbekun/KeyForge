use std::env;

pub mod cli_mode;
pub mod commands;
pub mod context;
pub mod interpreter;
use crate::cli_mode::input_loop::input_loop;

fn main() {

    // getting arguments from command line
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let _ = input_loop();
    }
}