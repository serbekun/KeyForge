use std::env;
use colored::*;

mod utils;
mod key_forge;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        key_forge::cli_mode();
    } 
    if !args[0].is_empty() {
        key_forge::file_mode(&args[0]);
    }
    else {
        if let Err(e) = key_forge::interpret_arguments_from_command_line(&args) {
            println!("{}" ,format!("{}", e).red().bold());
            std::process::exit(1);
        }
    }
}