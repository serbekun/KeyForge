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
}