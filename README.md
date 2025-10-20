# Key Forge 🔑

Command-line tool for generating random data, managing variables, and automating tasks through a powerful scripting language with advanced control flow and arithmetic operations.

## Features

### 🔢 Random Number Generation
- Generate random integers or floating-point numbers within specified ranges
- **Example:** `get_random_num 1 100` or `get_random_num 0.5 5.5`

### 🔤 Random Character Generation
- Generate random lowercase or uppercase letters
- **Example:** `get_random_char` (lowercase) or `get_random_char 1` (uppercase)

### 🔄 Command Repetition
- Repeat commands multiple times for batch operations
- **Example:** `repeat 10 get_random_num 1 100`

### 💾 Advanced Variable Management
- Store and manage variables of different types (integers, floats, strings)
- Support for command substitution in variable assignment
- Arithmetic operations on variables (add, subtract, multiply, divide)
- String manipulation operations
- **Examples:**
  ```bash
  set my_var 42                          # Integer
  set my_float 3.14                      # Float
  set my_string "hello"                  # String
  set my_random $(get_random_num 1 100)  # Command result
  
  add my_var 10                          # Add 10 to variable
  mul my_var 2                           # Multiply variable by 2
  push_to_string_back my_string " world" # Append to string
  num_to_string str_var 42               # Convert number to string
  ```

### 🔄 Control Flow Operations
- **Conditional execution** with `if-then-else` statements
- **Loops** with `while` and `for` constructs
- **Loop control** with `break` and `continue`
- Support for both inline commands and multi-line blocks
- **Examples:**
  ```bash
  # Conditional execution
  if $x > 10 then print "Large" else print "Small"
  
  # While loop
  while $i < 5 do print $i
  
  # For loop
  for i in 1..5 do print $i
  
  # Block syntax
  for i in 1..3 do {
      print "Iteration $i"
      print $(get_random_num 1 10)
  }
  ```

### 📤 Advanced Output Control
- Print variables or literal values with colored output
- Write command output to files
- Variable substitution in strings
- **Examples:**
  ```bash
  print my_var                    # Print variable
  print "Hello $name"             # Print with variable substitution
  print 123                       # Print number
  to_file output.txt print $result # Write output to file
  ```

### 📁 File Operations
- Execute commands from script files
- Append command output to files
- Support for multi-line scripts and block commands

## Usage

### Interactive CLI Mode
Run without arguments to start the interactive command-line interface:
```bash
key_forge
```

### File Mode
Execute commands from a script file:
```bash
key_forge script.txt
```

### Argument Mode
Execute commands directly from command line arguments:
```bash
key_forge arg "set x 10" "print x" "add x 5" "print x"
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `get_random_num <min> <max>` | Generate random number | `get_random_num 1 100` |
| `get_random_char [mode]` | Generate random character (0=lower, 1=upper) | `get_random_char 1` |
| `repeat <count> <command>` | Repeat command N times | `repeat 5 get_random_char` |
| `set <name> <value>` | Set variable | `set count 42` |
| `print <value>` | Print variable or literal | `print count` |
| `vl [mode]` | Show variable list (i=int, f=float, s=string) | `vl i` |
| `rm <name>` | Remove variable | `rm x` |
| `add <var> <value>` | Add value to variable | `add x 10` |
| `sub <var> <value>` | Subtract value from variable | `sub x 5` |
| `mul <var> <value>` | Multiply variable by value | `mul x 2` |
| `div <var> <value>` | Divide variable by value | `div x 2` |
| `num_to_string <target> <source>` | Convert number to string | `num_to_string str 42` |
| `push_to_string_back <var> <value>` | Append to string variable | `push_to_string_back s "!"` |
| `if <cond> then <cmd> [else <cmd>]` | Conditional execution | `if $x > 0 then print "Positive"` |
| `while <cond> do <cmd>` | While loop | `while $i < 5 do print $i` |
| `for <var> in <start>..<end> do <cmd>` | For loop | `for i in 1..5 do print $i` |
| `break` | Break out of loop | `break` |
| `continue` | Continue to next iteration | `continue` |
| `execute_file <filename>` | Execute commands from file | `execute_file script.txt` |
| `to_file <file> <command>` | Write command output to file | `to_file out.txt print $result` |
| `clear` | Clear terminal screen | `clear` |
| `help [command]` | Show help | `help` or `help set` |
| `command_list` | List all commands | `command_list` |
| `exit [code]` / `quit [code]` | Exit program | `exit` or `exit 1` |

## Variable Types

The tool supports three variable types that are automatically detected:

- **Integer**: Whole numbers (`42`, `-10`, `0`)
- **Float**: Decimal numbers (`3.14`, `-2.5`, `0.0`)
- **String**: Text values (`"hello"`, `'world'`)

## Command Substitution

Use `$(command)` to capture command output in variable assignment and other operations:
```
set random_num $(get_random_num 1 100)
print $(get_random_char)
to_file output.txt $(repeat 5 get_random_num 1 10)
```

## Condition Syntax

Conditions support comparison operators and complex expressions:
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `and`, `or`
- **Examples**:
  ```
  if $x > 10 and $y < 20 then print "Valid"
  if $name == "admin" or $id == 1 then print "Special"
  ```

## Block Commands and Multi-line Scripts

You can write multi-line blocks using braces `{ ... }` for complex operations:

```bash
# Multi-line while loop
while $counter < 10 do {
    print "Counter: $counter"
    add counter 1
    print $(get_random_num 1 100)
}

# Multi-line for loop  
for i in 1..5 do {
    set square $(mul $i $i)
    print "Square of $i is $square"
}

# Multi-line if statement
if $score > 50 then {
    print "Passed!"
    set status "Success"
} else {
    print "Failed!"
    set status "Failure"
}
```

## Advanced Examples

### Generate Random Data File
```bash
# Save 50 random numbers to file
to_file numbers.txt repeat 50 get_random_num 1 1000

# Create a matrix of random numbers
to_file matrix.txt repeat 10 repeat 10 get_random_num 0 9
```

### Mathematical Operations
```bash
set x 10
set y 5
add x $y          # x = 15
mul x 2           # x = 30
div x 3           # x = 10
num_to_string str $x  # str = "10"
```

### Complex Scripting
Create `advanced_script.txt`:
```bash
# Generate multiple random strings
set result ""
for i in 1..5 do {
    set char $(get_random_char)
    push_to_string_back result $char
}
print "Random string: $result"

# Conditional processing
set value $(get_random_num 1 100)
if $value > 50 then {
    print "$value is large"
    to_file large.txt print $value
} else {
    print "$value is small" 
    to_file small.txt print $value
}
```

Run with:
```bash
key_forge advanced_script.txt
```

## Error Handling

- Clear error messages with colored output
- Type safety for variable operations
- Command validation and usage hints
- Proper error reporting for file operations and arithmetic
- Loop control safety (break/continue only in valid contexts)

## Building

```bash
cargo build --release
```

## Dependencies

- `rand`: Random number generation
- `colored`: Colored terminal output
- `lazy_static`: For global variable storage
- Standard library collections and synchronization primitives

## Exit Codes

- `exit` or `exit 0`: Normal exit
- `exit <code>`: Exit with specified code (e.g., `exit 1` for error)

## Notes

- Commands are case-sensitive
- Variables are globally scoped and thread-safe
- String values can be quoted with `"` or `'`
- The tool provides colored output for better readability
- Multi-line blocks must use proper brace matching
- Command substitution can be used in most contexts that accept values

---

**Key Forge** - Forge your keys and data with powerful scripting capabilities! 🔑✨