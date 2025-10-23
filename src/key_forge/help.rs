use colored::Colorize;

pub fn show_all_help() {
    println!("{}", "Commands of key_forge".green());
    println!("");

    println!("{}", "get_random_num : use for get random num with diapason".blue());
    println!("Examples:");
    println!(" get_random_num 1 100    - generates random integer between 1-100");
    println!(" get_random_num 0.5 5.5  - generates random float between 0.5-5.5");
    println!("");

    println!("{}", "get_random_char : use for get random char from alphabet".blue());
    println!("Examples:");
    println!(" get_random_char      - return random lowercase char example 'a'");
    println!(" get_random_char 1    - return random uppercase char example 'B'");
    println!("");

    println!("{}", "repeat : use for repeat one command n times".blue());
    println!("Examples:");
    println!(" repeat 10 get_random_num 1 100 - repeat command get_random_num 10 times");
    println!("");

    println!("{}", "set : use for set variable with value".blue());
    println!("Examples:");
    println!(" set my_var 42                      - set integer variable");
    println!(" set my_var 3.14                    - set float variable");
    println!(" set my_var \"hello\"               - set string variable");
    println!(" set my_var $(get_random_num 1 100) - set with command result");
    println!("");

    println!("{}", "print : use for print variable value or literal".blue());
    println!("Examples:");
    println!(" print my_var                  - print variable value");
    println!(" print \"Hello World\"         - print literal string");
    println!(" print 123                     - print literal number");
    println!(" print $(get_random_num 1 100) - print output command get_random_num");
    println!("");
    

    println!("{}", "exit/quit : exit the program".blue());
    println!("Examples:");
    println!(" exit                 - exit with code 0");
    println!(" exit 1              - exit with code 1");
    println!("");

    println!("{}", "vl : use for show variables list".blue());
    println!("Examples:");
    println!(" vl - show all variables");
    println!(" vl i - show only int variables");
    println!(" vl f - show only float variables");
    println!(" vl s - show only string variables");
    println!("");

    println!("{}", "execute_file : for execute commands in file".blue());
    println!("Examples:");
    println!(" execute_file filename.txt - execute command in file filename.txt");
    println!("");

    println!("{}", "to_file : use for write output to file".blue());
    println!("Examples:");
    println!(" to_file filename.txt $(get_random_num 1 100) - write to file output of get_random_num");
    println!("");

    println!("{}", "add : for add value to variable".blue());
    println!("Examples:");
    println!("add x 10                      - add 10 to variable x");
    println!("add x y                       - add other to x");
    println!("add x $(get_random_num 1 100) - add output of get_random_num");
    println!();
    
    println!("{}", "mul : for multiply values".blue());
    println!("Examples:");
    println!("mul x 5                          - multiply x * 3 and store in x");
    println!("mul x y                          - multiply variables x y and store in x");
    println!("mul result $(get_random_num 2 5) - multiply random number by 10");
    println!("");

    println!("{}", "div : for divide values".blue());
    println!("Examples:");
    println!("div x 2                              - divide 10 / 2 and store in x");
    println!("div x z                              - divide y / z and store in x");
    println!("div result 100 $(get_random_num 2 5) - divide 100 by random number");
    println!("");

    println!("{}", "push_to_string_back : append value to the end of a string variable".blue());
    println!("Examples:");
    println!(" push_to_string_back my_string \" appended text\" - append literal text");
    println!(" push_to_string_back my_string $(get_random_char) - append random character");
    println!(" push_to_string_back str1 str2 - append value of str2 to str1");
    println!("");

    println!("{}", "num_to_string : convert number to string and store in variable".blue());
    println!("Examples:");
    println!(" num_to_string str_var 42                    - convert number 42 to string");
    println!(" num_to_string str_var my_number_var         - convert variable value to string");
    println!(" num_to_string result $(get_random_num 1 100) - convert command output to string");
    println!("");

    println!("{}", "clear : clear screen".blue());
    println!("Examples");
    println!(" clear - clear screen");
    println!("");

    println!("{}", "if : if operator".blue());
    println!("Examples");
    println!(" if $x == 0 then print 'x = 0'");
    println!(" if $x == 0 then print 'x = 0' else print 'x != 0'");
    println!(" if $input == 'grn' then get_random_num 1 100");
    println!("");

    println!("{}", "for : for command");
    println!("Examples");
    println!(" for i in 1..5 do print $i");
    println!("");

    println!("{}", "while : while command");
    println!("Examples");
    println!(" while $i < 10 do execute_file my_program.kf");
    println!("");

    println!("{}", "save_state : save all variables to file".blue());
    println!("Examples:");
    println!(" save_state state.txt                    - save to state.txt");
    println!(" save_state $filename                   - save to variable filename");
    println!(" save_state $(get_random_char 1).txt    - save to random filename");
    println!("");

    println!("{}", "load_state : load variables from file".blue());
    println!("Examples:");
    println!(" load_state state.txt                   - load from state.txt");
    println!(" load_state $filename                   - load from variable filename");
    println!(" load_state $(echo state).txt           - load from command result");
    println!("");

    println!("{}", "base64_encode : encode string with base64".blue());
    println!("Examples:");
    println!(" set encode_string $(base64_encode string)");
    println!("");

    println!("{}", "base64_decode : decode string with base64".blue());
    println!("Examples:");
    println!(" set decode_string $(base64_decode encode_string)");
    println!("");

    println!("{}", "remove_string_char : remove one char in string by index");
    println!("Examples");
    println!(" remove_string_char string_variable index");
    println!("");

    println!("{}", "help : show this help message".blue());
}

pub fn show_command_list() {
    println!("{}: {}", "get_random_num".blue(), "use for get random num with diapason");
    println!("{}: {}", "repeat".blue(), "use for repeat one command n times");
    println!("{}: {}", "set".blue(), "use for set variable with value");
    println!("{}: {}", "print".blue(), "use for print variable value or literal");
    println!("{}: {}", "exit/quit".blue(), "exit the program");
    println!("{}: {}", "vl".blue(), "use for show variables list");
    println!("{}: {}", "execute_file".blue(), "for execute commands in file");
    println!("{}: {}", "to_file".blue(), "use for write output to file");
    println!("{}: {}", "add".blue(), "for add value to variable");
    println!("{}: {}", "mul".blue(), "for multiply values");
    println!("{}: {}", "div".blue(), "for divide values");
    println!("{}: {}", "if".blue(), "if operator");
    println!("{}: {}", "for".blue(), "for operator");
    println!("{}: {}", "while".blue(), "while operator");
    println!("{}: {}", "push_to_string_back".blue(), "for push to string back other string");
    println!("{}: {}", "num_to_string".blue(), "convert number to string and store in variable");
    println!("{}: {}", "base64_encode".blue(), "encode string with base64");
    println!("{}: {}", "base64_decode".blue(), "decode string with base64");
    println!("{}: {}", "remove_string_char".blue(), "remove one char in string by index")
}

pub fn show_command_help(name: &str) {
    match name {
        "get_random_num" => {
            println!("{}", "get_random_num <min> <max>".green());
            println!("Generate random integer or float in range [min, max) (min < max)");
            println!("Examples:");
            println!(" get_random_num 1 100");
            println!(" get_random_num 0.5 5.5");
        }
        "get_random_char" => {
            println!("{}", "get_random_char [mode]".green());
            println!("Return a random character from alphabet. mode=1 for uppercase");
            println!("Examples:");
            println!(" get_random_char");
            println!(" get_random_char 1");
        }
        "repeat" => {
            println!("{}", "repeat <count> <command>".green());
            println!("Execute <command> <count> times. Command can be a substitution using $(...) .");
            println!("Example: repeat 10 get_random_num 1 100");
        }
        "set" => {
            println!("{}", "set <name> <value>".green());
            println!("Set variable <name> to <value>. Value may be an int, float, quoted string,");
            println!("another variable (using $name), or a command substitution: $(command)");
            println!("Examples:");
            println!(" set my_var 42");
            println!(" set my_var \"hello\"");
            println!(" set my_var $(get_random_num 1 100)");
        }
        "print" => {
            println!("{}", "print <name or literal>".green());
            println!("Print variable value or literal. Variables can be referenced with $name inside strings.");
            println!("Examples:");
            println!(" print my_var");
            println!(" print \"Hello $name\"");
        }
        "vl" => {
            println!("{}", "vl [i|f|s]".green());
            println!("Show variables. i - ints, f - floats, s - strings. Without arg shows all.");
        }
        "to_file" => {
            println!("{}", "to_file <filename> <command...>".green());
            println!("Execute <command...> and append its output to <filename>. Command can be $(...) substitution.");
        }
        "add" | "sub" | "mul" | "div" => {
            println!("{}", "<op> <var> <value>".green());
            println!("Arithmetic operations on variables. Supported ops: add, sub, mul, div.");
            println!("Examples:");
            println!(" add x 1");
            println!(" mul x 2");
        }
        "num_to_string" => {
            println!("{}", "num_to_string <target_variable> <source>".green());
            println!("Convert number or variable to a string and store in target variable.");
        }
        "push_to_string_back" => {
            println!("{}", "push_to_string_back <variable_name> <value>".green());
            println!("Append value to end of string variable (creates variable if missing). Value may be variable or $(...) command.");
        }
        "if" => {
            println!("{}", "if <condition> then <command> [else <command>]".green());
            println!("Evaluate condition and execute then/else command. Conditions support ==, !=, <, >, <=, >= and and/or.");
        }
        "while" => {
            println!("{}", "while <condition> do <command|{ ... }>".green());
            println!("Execute loop while condition is true. Body may be a single command or a block {{ ... }}.");
        }
        "for" => {
            println!("{}", "for <var> in <start>..<end> do <command|{ ... }>".green());
            println!("Loop variable takes values start..end-1. Body may be single command or block {{ ... }}.");
        }
        "break" => {
            println!("{}", "break".green());
            println!("Break out of the nearest loop (only valid in interactive/file mode, not in assignments).");
        }
        "continue" => {
            println!("{}", "continue".green());
            println!("Skip to next iteration of the nearest loop (only valid in interactive/file mode).");
        }
        "help" => {
            println!("{}", "help [command]".green());
            println!("Show general help or help for a single command: help get_random_num");
        }

        "save_state" => {
            println!("{}", "save_state <filename>".green());
            println!("Save all variables to a file. Filename can be:");
            println!("  - direct string: save_state state.txt");
            println!("  - variable: save_state $filename");
            println!("  - command result: save_state $(command)");
            println!("");
        }

        "load_state" => {
            println!("{}", "load_state <filename>".green());
            println!("Load variables from file (replaces current state). Filename can be:");
            println!("  - direct string: load_state state.txt");
            println!("  - variable: load_state $filename");
            println!("  - command result: load_state $(command)");
            println!("");
        }

        "base64_encode" => {
            println!("{}", "base64_encode : encode string with base64".blue());
            println!("Examples:");
            println!(" set encode_string $(base64_encode string)");
            println!("");
        }

        "base64_decode" => {
            println!("{}", "base64_decode : decode string with base64".blue());
            println!("Examples:");
            println!(" set decode_string $(base64_decode encode_string)");
            println!("");            
        }

        "remove_string_char" => {
            println!("{}", "remove_string_char : remove one char in string by index");
            println!("Examples");
            println!(" remove_string_char string_variable index");
            println!("");
        }

        _ => {
            println!("No detailed help for '{}'. Use help to see available commands.", name);
        }
    }
}