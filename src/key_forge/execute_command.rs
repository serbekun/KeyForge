// Command dispatcher: this file delegates each command to a module under `commands/`

pub fn execute_command(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.is_empty() {
        return Ok(String::new());
    }

    match args[0].as_str() {
        "//" => Ok(String::new()),

        "get_random_num" => {
            return crate::key_forge::commands::get_random_num::run(args, capture_output);
        }

        "get_random_char" => {
            return crate::key_forge::commands::get_random_char::run(args, capture_output);
        }

        "quit" | "exit" => {
            return crate::key_forge::commands::quit_exit::run(args, capture_output);
        }

        "help" => {
            return crate::key_forge::commands::help_cmd::run(args, capture_output);
        }

        "command_list" => {
            return crate::key_forge::commands::command_list::run(args, capture_output);
        }

        "repeat" => {
            return crate::key_forge::commands::repeat::run(args, capture_output);
        }

        "set" => {
            return crate::key_forge::commands::set::run(args, capture_output);
        }
                
        "rm" => {
            return crate::key_forge::commands::rm::run(args, capture_output);
        }

        "print" => {
            return crate::key_forge::commands::print_cmd::run(args, capture_output);
        }

        "execute_file" => {
            return crate::key_forge::commands::execute_file::run(args, capture_output);
        }

        "vl" => {
            return crate::key_forge::commands::vl::run(args, capture_output);
        }

        "to_file" => {
            return crate::key_forge::commands::to_file::run(args, capture_output);
        }

        "add" | "sub" | "mul" | "div" => {
            return crate::key_forge::commands::arithmetic_op::run(args, capture_output);
        }

        "num_to_string" => {
            return crate::key_forge::commands::num_to_string::run(args, capture_output);
        }

        "push_to_string_back" => {
            return crate::key_forge::commands::push_to_string_back::run(args, capture_output);
        }
        
        "clear" => {
            return crate::key_forge::commands::clear_cmd::run(args, capture_output);
        }
        
        "if" => {
            return crate::key_forge::commands::if_cmd::run(args, capture_output);
        }

        "while" => {
            return crate::key_forge::commands::while_cmd::run(args, capture_output);
        }

        "for" => {
            return crate::key_forge::commands::for_cmd::run(args, capture_output);
        }

        "break" => {
            return crate::key_forge::commands::break_cmd::run(args, capture_output);
        }

        "continue" => {
            return crate::key_forge::commands::continue_cmd::run(args, capture_output);
        }

        "save_state" => {
            return crate::key_forge::commands::save_state::run(args, capture_output);
        }

        "load_state" => {
            return crate::key_forge::commands::load_state::run(args, capture_output);
        }

        "base64_encode" => {
            return crate::key_forge::commands::base64_encode::run(args, capture_output);
        }

        "base64_decode" => {
            return crate::key_forge::commands::base64_decode::run(args, capture_output);
        }

        "remove_string_char" => {
            return crate::key_forge::commands::remove_string_char::run(args, capture_output);
        }

        "push" => {
            return crate::key_forge::commands::push::run(args, capture_output);
        }

        "pop" => {
            return crate::key_forge::commands::pop::run(args, capture_output);
        }

        "len" => {
            return crate::key_forge::commands::len::run(args, capture_output);
        }

        "keys" => {
            return crate::key_forge::commands::keys::run(args, capture_output);
        }

        "values" => {
            return crate::key_forge::commands::values::run(args, capture_output);
        }

        "get" => {
            return crate::key_forge::commands::get::run(args, capture_output);
        }

        "multi_arg_command" => {
            return crate::key_forge::commands::multi_arg_command::run(args, capture_output);
        }

        "write_file" => {
            return crate::key_forge::commands::write_file::run(args, capture_output);
        }

        "read_file" => {
            return crate::key_forge::commands::read_file::run(args, capture_output);
        }

        _ => {
            if capture_output {
                Err(format!(
                    "Unknown command '{}'",
                    args[0]
                ))
            } else {
                Err(format!("Unknown command {}", args[0]))
            }
        }
    }
}
