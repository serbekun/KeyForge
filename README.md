# Key Forge

Command-line tool for generating random data, managing variables, and automating tasks through a scripting language with advanced control flow, arithmetic operations, and collection types.

## Features

### Variable Management
- **set** - Create typed variables (int, float, string, bool, binary)
- **vl** - List all variables with their types
- **rm** - Delete a variable

### Arithmetic Operations
- **add** - Add numbers or concatenate strings
- **sub** - Subtract numbers
- **mul** - Multiply numbers
- **div** - Divide numbers

### Random Generation
- **random-num** - Generate random integer or float
- **random-char** - Generate single random character
- **random-string** - Generate random string of specified length

### File I/O
- **write** - Write content to file (append or overwrite)
- **read_file** - Read and display file content

### Encoding
- **base64-encode** - Encode value to base64
- **base64-decode** - Decode base64 value

### State Management
- **save-state** - Save all variables to JSON file
- **load-state** - Load variables from JSON file

### Utility
- **clear** - Clear terminal screen
- **help** - Show help (globally or for specific command)
- **exit** - Exit program

## Substitution Syntax

KeyForge supports variable and command substitution:

- `$variable_name` - Returns the value of a variable
- `$(command args)` - Executes a command and uses its result as a string

When commands are used directly (without substitution), output is printed to console. When used inside substitution, the result is passed as a string without automatic output.

## Examples

```bash
# Set variables
set int x 10
set int y 5

# Use variables in operations
add $x $y  # Output: 15

# String manipulation
set string name Sergei
add Hello_ $name  # Output: Hello_Sergei

# Substitution in commands
write file.txt $(random-string 10) w
set int r $(random-num 1 100)

# List variables
vl

# Save and load state
save-state backup.json
load-state backup.json

# Encoding
base64-encode $name
base64-decode SGVyZ2Vp
```

## Type System

All values are stored with their types:
- **int**: Integer numbers (i64)
- **float**: Floating-point numbers (f64)
- **string**: Text strings
- **bool**: Boolean values (true/false)
- **binary**: Binary data

Type conversions happen automatically when needed (e.g., arithmetic operations).

## Building

```bash
cargo build --release
```

## Running

Interactive mode:
```bash
./target/release/key_forge
```
