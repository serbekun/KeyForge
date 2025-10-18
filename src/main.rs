use std::env;
use colored::Colorize;

mod key_forge;
mod utils;

use key_forge::{cli_mode, file_mode, interpret_arguments_from_command_line};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        cli_mode();
    } else if args[0] == "arg" {
        args.remove(0);
        if let Err(e) = interpret_arguments_from_command_line(&args) {
            println!("{}", format!("{}", e).red().bold());
            std::process::exit(1);
        }
    } else if !args[0].is_empty() {
        file_mode(&args[0]);
    }
}