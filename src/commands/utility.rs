use crate::interpreter::Command;
use crate::context::Context;

pub struct ClearCommand;

impl Command for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }

    fn execute(&self, _args: &[String], _context: &mut Context) -> Result<String, String> {
        print!("\x1B[2J\x1B[1;1H");
        Ok(String::new())
    }
}

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: &[String], _context: &mut Context) -> Result<String, String> {
        std::process::exit(0);
    }
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn execute(&self, args: &[String], _context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Ok(get_general_help());
        }

        let command = &args[0];
        Ok(match command.as_str() {
            "set" => "set <type> <name> <value>\nSets a variable. Types: int, float, string, bool, binary".to_string(),
            "vl" => "vl\nLists all variables".to_string(),
            "rm" => "rm <name>\nRemoves a variable".to_string(),
            "add" => "add <val1> <val2>\nAdds two values (concatenates strings, sums numbers)".to_string(),
            "sub" => "sub <val1> <val2>\nSubtracts val2 from val1".to_string(),
            "mul" => "mul <val1> <val2>\nMultiplies two values".to_string(),
            "div" => "div <val1> <val2>\nDivides val1 by val2".to_string(),
            "random-num" => "random-num <min> <max>\nGenerates random number between min and max".to_string(),
            "random-char" => "random-char\nGenerates random character".to_string(),
            "random-string" => "random-string <len>\nGenerates random string of length".to_string(),
            "write" => "write <filename> <content> <mode>\nWrites content to file. Mode: w (overwrite) or a (append)".to_string(),
            "read_file" => "read_file <filename>\nReads and returns file content".to_string(),
            "base64-encode" => "base64-encode <value>\nEncodes value to base64".to_string(),
            "base64-decode" => "base64-decode <value>\nDecodes base64 value".to_string(),
            "save-state" => "save-state <filename>\nSaves all variables to JSON file".to_string(),
            "load-state" => "load-state <filename>\nLoads variables from JSON file".to_string(),
            "clear" => "clear\nClears the terminal screen".to_string(),
            "exit" => "exit\nExits the program".to_string(),
            "help" => "help [command]\nShows help for all commands or specific command".to_string(),
            _ => format!("Unknown command: {}", command),
        })
    }
}

fn get_general_help() -> String {
    "KeyForge CLI - Command Reference

Variable Management:
  set <type> <name> <value>    - Set variable (types: int, float, string, bool, binary)
  vl                            - List all variables
  rm <name>                     - Remove variable

Arithmetic (return string results):
  add <val1> <val2>            - Add/concatenate
  sub <val1> <val2>            - Subtract
  mul <val1> <val2>            - Multiply
  div <val1> <val2>            - Divide

Random Generation:
  random-num <min> <max>       - Random number
  random-char                  - Random character
  random-string <len>          - Random string

File I/O:
  write <file> <content> <mode> - Write file (w=overwrite, a=append)
  read_file <file>             - Read file

Encoding:
  base64-encode <value>        - Encode to base64
  base64-decode <value>        - Decode from base64

State:
  save-state <file>            - Save variables to JSON
  load-state <file>            - Load variables from JSON

Utility:
  clear                         - Clear screen
  exit                          - Exit program
  help [cmd]                    - Show help

Substitution:
  $variable                     - Get variable value
  $(command args)               - Execute command and use result

Example:
  set int x 10
  set string name Sergei
  add $x 5
  add Hello_ $name
  write file.txt $(random-string 10) w
".to_string()
}
