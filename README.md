# Key Forge 🔑

Command-line tool for generating random data, managing variables, and automating tasks through a powerful scripting language with advanced control flow, arithmetic operations, and collection types.

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
- Store and manage variables of different types (integers, floats, strings, arrays, dictionaries)
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

### 🗂️ Array and Dictionary Support
- **Arrays**: Ordered collections of values with push/pop operations
- **Dictionaries**: Key-value pairs for structured data storage
- **Collection Operations**: Length checking, element access, iteration
- **Examples:**
  ```bash
  # Arrays
  set numbers [1, 2, 3, 4]
  push numbers 5
  get numbers 0
  set numbers 0 10
  len numbers
  pop numbers

  # Dictionaries  
  set person {name: "John", age: 30, active: true}
  set person city "New York"
  get person name
  keys person
  values person
  len person
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
- Save and load variable state to/from files

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
| `vl [mode]` | Show variable list (i=int, f=float, s=string, a=array, d=dict) | `vl a` |
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
| `save_state <filename>` | Save variable state | `save_state $filename` |
| `load_state <filename>` | Load variable state | `load_state $filename` |
| `base64_encode` | Encode string in base64 | `base64_encode string` |
| `base64_decode` | Decode base64 string | `base64_decode string` |
| `remove_string_char` | Remove string char by index | `remove_string_char string 2` |
| `push <array> <value>` | Add element to array | `push numbers 5` |
| `pop <array>` | Remove and return last element | `pop numbers` |
| `len <collection>` | Get length of array/dict/string | `len numbers` |
| `keys <dict>` | Get all dictionary keys | `keys person` |
| `values <dict>` | Get all dictionary values | `values person` |
| `get <collection> <key/index>` | Get element from array/dict | `get numbers 0` |
| `set <collection> <key/index> <value>` | Set element in array/dict | `set numbers 0 10` |
| `exit [code]` / `quit [code]` | Exit program | `exit` or `exit 1` |

## Variable Types

The tool supports five variable types that are automatically detected:

- **Integer**: Whole numbers (`42`, `-10`, `0`)
- **Float**: Decimal numbers (`3.14`, `-2.5`, `0.0`)
- **String**: Text values (`"hello"`, `'world'`)
- **Array**: Ordered collections (`[1, 2, "hello", 3.14]`)
- **Dictionary**: Key-value pairs (`{name: "John", age: 30, active: true}`)

## Command Substitution

Use `$(command)` to capture command output in variable assignment and other operations:
```
set random_num $(get_random_num 1 100)
print $(get_random_char)
to_file output.txt $(repeat 5 get_random_num 1 10)
```

## Collection Syntax

### Arrays
Arrays are ordered collections that support indexing and various operations:
```bash
# Create array
set fruits ["apple", "banana", "cherry"]

# Access elements
get fruits 0          # Returns "apple"
set fruits 1 "orange" # Change "banana" to "orange"

# Array operations
push fruits "grape"   # Add to end
pop fruits            # Remove and return last element
len fruits            # Get length (3)
```

### Dictionaries
Dictionaries store key-value pairs for structured data:
```bash
# Create dictionary
set user {name: "Alice", age: 25, active: true}

# Access values
get user name         # Returns "Alice"
set user age 26       # Update age to 26

# Dictionary operations
keys user             # Returns ["name", "age", "active"]
values user           # Returns ["Alice", 26, true]
len user              # Get number of pairs (3)
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

### Array Processing
```bash
# Create and process array
set scores [85, 92, 78, 96, 88]
set total 0

for i in 0..$(len scores) do {
    get scores $i
    add total $result
}

set average $(div $total $(len scores))
print "Average score: $average"
```

### Dictionary Operations
```bash
# User management system
set users [
    {name: "Alice", role: "admin", level: 5},
    {name: "Bob", role: "user", level: 3},
    {name: "Charlie", role: "moderator", level: 4}
]

# Find all admins
set admin_count 0
for i in 0..$(len users) do {
    get users $i
    set user $result
    get user role
    if $result == "admin" then {
        add admin_count 1
        get user name
        print "Admin: $result"
    }
}

print "Total admins: $admin_count"
```

### Complex Scripting
Create `advanced_script.txt`:
```bash
# Generate multiple random strings
set result ""
for i in 1..5 do {
    set char $(get_random_char)
    push_to_string_back result char
}
print "Random string: $result"

# Conditional processing with collections
set data {values: [10, 20, 30, 40, 50], threshold: 25}
set high_values []

get data values
set values $result
for i in 0..$(len values) do {
    get values $i
    if $result > $(get data threshold) then {
        push high_values $result
    }
}

print "Values above threshold: $high_values"
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
- Collection bounds checking and type validation

## Building

```bash
cargo build --release
```

## Dependencies

- `rand`: Random number generation
- `colored`: Colored terminal output
- `lazy_static`: For global variable storage
- `serde`: Serialization for state saving/loading
- `serde_json`: JSON support for collections
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
- Arrays and dictionaries support nested structures
- Collection operations maintain type safety

---

**Key Forge** - Forge your keys and data with powerful scripting capabilities! 🔑✨