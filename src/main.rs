use std::env;
use colored::*;

mod utils;
mod key_forge;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        key_forge::cli_mode();
    } 
    else if args[0] == "arg" {
        args.remove(0);
        if let Err(e) = key_forge::interpret_arguments_from_command_line(&args) {
            println!("{}" ,format!("{}", e).red().bold());
            std::process::exit(1);
        }
    }
    else if !args[0].is_empty() {
        key_forge::file_mode(&args[0]);
    }
}