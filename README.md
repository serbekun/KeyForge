# Key Forge 🔑

command-line tool for generating random data, managing variables, and automating tasks through a simple scripting language.

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

### 💾 Variable Management
- Store and manage variables of different types (integers, floats, strings)
- Support for command substitution in variable assignment
- **Examples:**
  ```bash
  set my_var 42                          # Integer
  set my_float 3.14                      # Float
  set my_string "hello"                  # String
  set my_random $(get_random_num 1 100)  # Command result
  ```

### 📤 Output Control
- Print variables or literal values with colored output
- **Examples:**
  ```bash
  print my_var                    # Print variable
  print "Hello World"             # Print literal
  print 123                       # Print number
  ```

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

### Direct Command Execution
Run single commands directly (though the current implementation primarily supports file and interactive modes):
```bash
key_forge "get_random_num 1 100"
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `get_random_num <min> <max>` | Generate random number | `get_random_num 1 100` |
| `get_random_char [mode]` | Generate random character (0=lower, 1=upper) | `get_random_char 1` |
| `repeat <count> <command>` | Repeat command N times | `repeat 5 get_random_char` |
| `set <name> <value>` | Set variable | `set count 42` |
| `print <value>` | Print variable or literal | `print count` |
| `vl <mode>` | print variable list i - int, f - float, s - string | `vl i` |
| `rm <name>` | remove variable | `rm x` |
| `help` | Show help message | `help` |
| `exit [code]` | Exit program | `exit` or `exit 1` |
| `quit [code]` | Exit program | `quit` |

## Variable Types

The tool supports three variable types that are automatically detected:

- **Integer**: Whole numbers (`42`, `-10`, `0`)
- **Float**: Decimal numbers (`3.14`, `-2.5`, `0.0`)
- **String**: Text values (`"hello"`, `'world'`)

## Command Substitution

Use `$(command)` to capture command output in variable assignment:
```
set random_num $(get_random_num 1 100)
set random_char $(get_random_char 1)
```
## Fun commands
```
// save 50 random number with diapason from 1 to 1000 to file number.txt
to_file numbers.txt repeat 50 get_random_num 1 1000

// print to terminal random numbers
repeat 10 print $(get_random_num 1 6)

// create matrix
to_file matrix.txt repeat 10 repeat 10 get_random_num 0 9

// look to the files numbers.txt and matrix.txt
```


## Error Handling

- Clear error messages with colored output
- Type safety for variable operations
- Command validation and usage hints
- Line-number tracking in file mode

## Building

```bash
cargo build --release
```

## Dependencies

- `rand`: Random number generation
- `colored`: Colored terminal output
- Standard library collections and synchronization primitives

## Examples

### Basic Usage
```bash
> set iterations 5
> repeat $iterations get_random_num 1 50
> print iterations
```

### Advanced Scripting
Create a script file `generate_keys.txt`:
```
set length 10
set count 5
repeat $count get_random_char
print "Generated $count random characters"
```

Run with:
```bash
key_forge generate_keys.txt
```

## Exit Codes

- `exit` or `exit 0`: Normal exit
- `exit <code>`: Exit with specified code (e.g., `exit 1` for error)

## Notes

- Commands are case-sensitive
- Variables are globally scoped and thread-safe
- String values can be quoted with `"` or `'`
- The tool provides colored output for better readability
- Debug information is shown when setting variables with command substitution

---

**Key Forge** - Forge your keys and data with ease! 🔑✨