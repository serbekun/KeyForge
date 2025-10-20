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

    println!("{}", "help : show this help message".blue());
}

pub fn show_command_list() {
    println!("{}", "get_random_num : use for get random num with diapason".blue());
    println!("{}", "repeat : use for repeat one command n times".blue());
    println!("{}", "set : use for set variable with value".blue());
    println!("{}", "print : use for print variable value or literal".blue());
    println!("{}", "exit/quit : exit the program".blue());
    println!("{}", "vl : use for show variables list".blue());
    println!("{}", "execute_file : for execute commands in file".blue());
    println!("{}", "to_file : use for write output to file".blue());
    println!("{}", "add : for add value to variable".blue());
    println!("{}", "mul : for multiply values".blue());
    println!("{}", "div : for divide values".blue());
    println!("{}", "num_to_string : convert number to string and store in variable".blue());
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
        _ => {
            println!("No detailed help for '{}'. Use help to see available commands.", name);
        }
    }
}