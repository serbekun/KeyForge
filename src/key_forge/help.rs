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
    println!("{}", "set <collection_name> <key/index> <value>".green());
    println!("Set a value in an array (by index) or dictionary (by key).");
    println!("Examples:");
    println!(" set my_array 0 42          - set array element at index 0");
    println!(" set my_dict age 31         - set dictionary value for key 'age'");
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
    println!(" set str $(num_to_string 42)                        - convert number 42 to string");
    println!(" set str_my_number_var num_to_string $my_number_var - convert variable value to string");
    println!(" num_to_string $(get_random_num 1 100)              - convert command output to string");
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

    println!("{}", "Array and Dictionary Support".blue().bold());
    println!("");
    
    println!("{}", "Arrays: ordered collections of values".blue());
    println!("Examples:");
    println!(" set arr [1, 2, 3, 4]                - create array with values");
    println!(" push arr 5                          - add element to end of array");
    println!(" pop arr                             - remove and return last element");
    println!(" get arr 0                           - get element at index 0");
    println!(" set arr 0 10                        - set element at index 0 to 10");
    println!(" len arr                             - get length of array");
    println!("");

    println!("{}", "Dictionaries: key-value pairs".blue());
    println!("Examples:");
    println!(" set dict {{name: \"John\", age: 30}}   - create dictionary");
    println!(" set dict name \"Jane\"               - set value for key");
    println!(" get dict name                       - get value for key");
    println!(" keys dict                           - get all keys");
    println!(" values dict                         - get all values");
    println!(" len dict                            - get number of key-value pairs");
    println!("");

    println!("{}", "write_file : write content to file with mode".blue());
    println!("Examples:");
    println!(" write_file \"output.txt\" \"Hello World\" \"w\"    - overwrite file");
    println!(" write_file \"log.txt\" \"New entry\" \"a\"         - append to file");
    println!(" write_file $filename $content \"w\"               - use variables");
    println!(" write_file test.txt $(get_random_num 1 100) \"w\" - use command output");
    println!("");

    println!("{}", "read_file : read content from file".blue());
    println!("Examples:");
    println!(" read_file \"data.txt\"                          - read and print file content");
    println!(" set content $(read_file \"config.txt\")         - read file into variable");
    println!(" read_file $filename                            - read using variable");
    println!(" read_file $(echo \"file\").txt                 - read using command output");
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
    println!("{}: {}", "num_to_string".blue(), "convert number to string and return string");
    println!("{}: {}", "base64_encode".blue(), "encode string with base64");
    println!("{}: {}", "base64_decode".blue(), "decode string with base64");
    println!("{}: {}", "remove_string_char".blue(), "remove one char in string by index");
    println!("{}: {}", "push".blue(), "add element to array");
    println!("{}: {}", "pop".blue(), "remove and return last element from array");
    println!("{}: {}", "len".blue(), "get length of array, dictionary, or string");
    println!("{}: {}", "keys".blue(), "get all keys from dictionary");
    println!("{}: {}", "values".blue(), "get all values from dictionary");
    println!("{}: {}", "get".blue(), "get element from array or dictionary");
    println!("{}: {}", "write_file".blue(), "write content to file with mode selection");
    println!("{}: {}", "read_file".blue(), "read content from file");
    println!("{}: {}", "help".blue(), "show all commands");
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
            println!("{}", "num_to_string : convert number to string and store in variable".blue());
            println!("Examples:");
            println!(" set str $(num_to_string 42)                        - convert number 42 to string");
            println!(" set str_my_number_var num_to_string $my_number_var - convert variable value to string");
            println!(" num_to_string $(get_random_num 1 100)              - convert command output to string");
            println!("");
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

        
        "push" => {
            println!("{}", "push <array_name> <value>".green());
            println!("Add an element to the end of an array. Creates the array if it doesn't exist.");
            println!("Examples:");
            println!(" push my_array 42");
            println!(" push my_array \"hello\"");
            println!(" push my_array $(get_random_num 1 100)");
        }
        
        "pop" => {
            println!("{}", "pop <array_name>".green());
            println!("Remove and return the last element of an array.");
            println!("Examples:");
            println!(" pop my_array");
        }
        
        "len" => {
            println!("{}", "len <variable_name>".green());
            println!("Get the length of an array, dictionary, or string.");
            println!("Examples:");
            println!(" len my_array    - number of elements in array");
            println!(" len my_dict     - number of key-value pairs");
            println!(" len my_string   - number of characters");
        }
        
        "keys" => {
            println!("{}", "keys <dict_name>".green());
            println!("Get all keys from a dictionary as an array.");
            println!("Examples:");
            println!(" keys my_dict");
        }
        
        "values" => {
            println!("{}", "values <dict_name>".green());
            println!("Get all values from a dictionary as an array.");
            println!("Examples:");
            println!(" values my_dict");
        }
        
        "get" => {
            println!("{}", "get <collection_name> <key/index>".green());
            println!("Get a value from an array (by index) or dictionary (by key).");
            println!("Examples:");
            println!(" get my_array 0     - get first element of array");
            println!(" get my_dict name   - get value for key 'name'");
        }

        "write_file" => {
            println!("{}", "write_file <filename> <content> <append>".green());
            println!("Write content to file. Supports variables ($var) and command substitution ($(...)).");
            println!("");
            println!("Parameters:");
            println!("  filename - can be string, variable, or command output");
            println!("  content  - can be string, variable, or command output");
            println!("  append   - 'w' to overwrite, 'a' to append to file");
            println!("");
            println!("Examples:");
            println!("  write_file \"log.txt\" \"New message\" \"a\"      - append to file");
            println!("  write_file $fname \"Hello\" \"w\"               - use variable for filename");
            println!("  write_file out.txt $(get_random_char) \"w\"     - use command output");
            println!("  write_file data.txt $content \"w\"              - use variable for content");
        }
        
        "read_file" => {
            println!("{}", "read_file <filename>".green());
            println!("Read entire file content. Supports variables ($var) and command substitution ($(...)).");
            println!("Returns file content as string that can be captured in variable.");
            println!("");
            println!("Examples:");
            println!("  read_file \"config.txt\"                    - read and print file");
            println!("  set data $(read_file \"data.json\")         - store file content in variable");
            println!("  read_file $filename                        - read using variable");
            println!("  read_file $(echo \"file\").txt             - read using command output");
            println!("  print $(read_file \"notes.txt\")           - print file content directly");
        }

        _ => {
            println!("No detailed help for '{}'. Use help to see available commands.", name);
        }
    }
}