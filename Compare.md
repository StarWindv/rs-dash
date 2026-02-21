# rs-dash vs Standard Dash Comparison

## Overview

This document compares the current implementation of rs-dash (Rust reimplementation) with the standard dash shell (Debian Almquist Shell). The goal is to identify missing features and functionality gaps.

## Current Status (rs-dash v0.1.0)

### ✅ Implemented Features

#### 1. Core Shell Infrastructure
- Basic command line parsing
- Interactive mode support
- Script file execution (`-c` flag)
- Environment variable management
- Current directory tracking

#### 2. Built-in Commands
- `cd` - Change directory (with `-` support for OLDPWD)
- `pwd` - Print working directory
- `echo` - Print arguments
- `exit` - Exit shell with status code
- `true`/`false` - Success/failure commands
- `help` - Display help information

#### 3. Command Execution
- External command execution with PATH searching
- Cross-platform support (Windows/Linux)
- Basic error handling (command not found, file errors)

#### 4. Command Separators
- `;` - Sequential command execution
- `&&` - Execute next command only if previous succeeded
- `||` - Execute next command only if previous failed

#### 5. Pipelines
- Basic `|` operator support
- Command chaining (simplified implementation)

#### 6. Redirections
- `>` - Output redirection (create/truncate)
- `>>` - Append redirection
- `<` - Input redirection

#### 7. Variables
- Simple assignment: `VAR=value`
- Basic expansion: `$VAR`
- Special variables: `$?`, `$$`, `$0`

#### 8. Command Substitution
- Basic `$(command)` syntax support

## ❌ Missing Features (Compared to Standard Dash)

### 1. POSIX Shell Grammar
- **Full grammar parsing**: rs-dash uses simplified parsing vs dash's complete yacc-based parser
- **Complex word expansions**: Missing many expansion forms
- **Complete token recognition**: Limited token types compared to dash

### 2. Built-in Commands (Missing from dash)
Standard dash has 30+ built-in commands. rs-dash is missing:

#### POSIX Special Builtins (must be built-in):
- `.` (dot) - Execute commands from file
- `:` (colon) - Null command
- `break` - Exit from loops
- `continue` - Continue loop iteration
- `eval` - Evaluate arguments as shell commands
- `exec` - Replace shell with command
- `export` - Set export attribute for variables
- `readonly` - Make variables readonly
- `return` - Return from function
- `set` - Set shell options
- `shift` - Shift positional parameters
- `times` - Print process times
- `trap` - Handle signals
- `unset` - Unset variables

#### POSIX Standard Utilities (usually built-in):
- `alias`/`unalias` - Command aliasing
- `bg` - Background job control
- `command` - Execute simple command
- `fc` - Command history editor
- `fg` - Foreground job control
- `getopts` - Parse positional parameters
- `hash` - Remember command locations
- `jobs` - List active jobs
- `kill` - Send signals to processes
- `read` - Read input
- `test`/`[` - Test conditions
- `type` - Describe command type
- `umask` - Set file creation mask
- `ulimit` - Control resource limits
- `wait` - Wait for process completion

### 3. Variable System
- **Parameter expansion**: Missing `${VAR}`, `${VAR:-default}`, `${VAR:=default}`, `${VAR:?message}`, `${VAR:+value}`
- **String operations**: `${#VAR}`, `${VAR%pattern}`, `${VAR%%pattern}`, `${VAR#pattern}`, `${VAR##pattern}`
- **Array variables**: Not supported
- **Positional parameters**: `$1`, `$2`, ..., `$@`, `$*`
- **Special parameters**: `$-`, `$!`, `$#`
- **Variable attributes**: Readonly, export, integer, array
- **Variable scoping**: Local vs global variables

### 4. Command Substitution
- **Backtick syntax**: `` `command` `` not supported
- **Nested substitutions**: Limited support
- **Quoting within substitutions**: Complex cases not handled

### 5. Arithmetic Expansion
- **Missing entirely**: `$((expression))` syntax
- **Integer arithmetic**: All arithmetic operations missing
- **Bit operations**: `&`, `|`, `^`, `~`, `<<`, `>>`
- **Ternary operator**: `? :`
- **Comma operator**: `,`

### 6. Process Control & Job Management
- **Job control**: `&`, `jobs`, `fg`, `bg`, `wait`
- **Process groups**: Not implemented
- **Signal handling**: `trap`, signal masks
- **Subshell execution**: `(command)`
- **Process substitution**: Not supported

### 7. Shell Scripting Features
- **Functions**: `name() { ... }` syntax
- **Compound commands**: `if`, `for`, `while`, `until`, `case`
- **Here documents**: `<<` and `<<-` (here-doc and here-string)
- **Conditional execution**: Full `if-then-else-fi`, `case-esac`
- **Loop control**: `break`, `continue` with levels
- **Select statement**: `select var in list`

### 8. Redirections (Advanced)
- **File descriptor redirection**: `>&`, `<&`, `>&-`
- **Duplication**: `n>&m`, `n<&m`
- **Here documents with expansion**: `<< "EOF"`
- **Process substitution**: `<(command)`, `>(command)`

### 9. Quoting Rules
- **ANSI-C quoting**: `$'...'` not supported
- **Locale-specific quoting**: Limited support
- **Quote removal timing**: Complex cases not handled correctly

### 10. Shell Options
- **set options**: `-e`, `-u`, `-x`, `-o option`
- **shopt options**: Not implemented
- **Interactive options**: `-i`, `-m`, `-s`

### 11. History Features
- **Command history**: Not implemented
- **History expansion**: `!`, `!!`, `!n`, `!-n`, `!string`
- **fc command**: History editor

### 12. Completion
- **Tab completion**: Not implemented
- **Programmable completion**: Not supported

### 13. Performance & Optimization
- **Command hashing**: `hash` built-in missing
- **Fast path execution**: Optimizations for simple commands
- **Builtin preference**: Builtins vs external commands

### 14. Error Handling & Diagnostics
- **Line numbers in errors**: Not implemented
- **Better error messages**: Limited diagnostics
- **Debug mode**: `set -x` equivalent missing

### 15. Portability & Standards
- **POSIX compliance**: Limited compliance
- **Shebang handling**: Basic only
- **Signal number portability**: Not implemented

## Technical Architecture Differences

### 1. Parser Architecture
- **dash**: Yacc-based parser with complete grammar
- **rs-dash**: Hand-written recursive descent with limited grammar

### 2. Execution Model
- **dash**: Fork-exec model with job control
- **rs-dash**: Simplified process spawning

### 3. Memory Management
- **dash**: Custom allocator with arenas
- **rs-dash**: Rust ownership model (safer but different)

### 4. Variable System
- **dash**: Complex variable table with attributes
- **rs-dash**: Simple HashMap<String, String>

### 5. Built-in Commands
- **dash**: Table-driven with function pointers
- **rs-dash**: Match statement with hardcoded functions

## Test Coverage Gaps

Based on dash test suite analysis, rs-dash is missing tests for:

1. **Grammar tests**: Complex parsing cases
2. **Expansion tests**: All parameter expansion forms
3. **Arithmetic tests**: All arithmetic expressions
4. **Redirection tests**: Advanced redirection forms
5. **Job control tests**: Background/foreground jobs
6. **Signal tests**: Trap and signal handling
7. **POSIX compliance tests**: Standards conformance
8. **Performance tests**: Execution speed comparisons

## Priority Areas for Implementation

### High Priority (Core POSIX compliance)
1. Full POSIX grammar parsing
2. Parameter expansion `${...}`
3. Arithmetic expansion `$((...))`
4. Functions and control structures
5. Positional parameters

### Medium Priority (Shell usability)
1. Job control (`&`, `jobs`, `fg`, `bg`)
2. History mechanism
3. Additional builtins (`test`, `read`, `printf`)
4. Here documents
5. Signal handling (`trap`)

### Low Priority (Advanced features)
1. Arrays
2. Process substitution
3. Coprocesses
4. Advanced completion
5. Internationalization

## Notes

- rs-dash has a good foundation with basic shell functionality
- The Rust implementation provides memory safety advantages
- Cross-platform support is a strength of rs-dash
- Many missing features are complex but well-documented in dash source
- The modular architecture of rs-dash should facilitate adding missing features

## References

1. dash source code in `../c-dash/`
2. POSIX Shell Command Language: IEEE Std 1003.1-2017
3. dash man page and documentation
4. Test suites in `test/` directory## Current Implementation Issues (Based on Code Review & Testing)

### 1. Command Substitution Problems
- **Command substitution `$(pwd)` works but parsing issues exist**: Based on quick tests, command substitution actually works but may have edge cases
- **Nested substitutions**: Implementation exists but needs testing
- **Backtick syntax**: `` `command` `` not supported

### 2. Variable Expansion Issues
- **`$?` returns numeric exit code correctly**: Based on quick tests, this is working
- **Variable assignment and expansion works**: `MYVAR=hello; echo $MYVAR` works in tests
- **Parameter expansion `${...}` partially implemented**: Basic forms work but advanced forms need testing
- **Positional parameters**: `$1`, `$2`, etc. have basic support but need more testing

### 3. Parser Implementation
- **No panic found in current code**: The previously reported panic at line 91 appears to be `None => break,` which is not a panic
- **Simplified parser**: Hand-written recursive descent vs dash's yacc-based parser
- **Limited grammar support**: Missing complex shell grammar features
- **Quoting handling**: Basic but may have edge cases

### 4. External Command Issues
- **Cross-platform PATH searching**: Implementation exists for both Windows and Unix
- **Command not found handling**: Returns exit code 127 as expected
- **Builtin vs external command distinction**: Working correctly

### 5. Exit Status Handling
- **Exit codes work correctly**: Based on tests, exit status propagation works
- **`$?` expansion returns numeric values**: Working as expected
- **Exit code from external commands**: Properly captured and stored

### 6. Pipeline Implementation
- **Basic pipeline support**: `|` operator works for simple cases
- **True pipe implementation needed**: Current implementation may not use actual OS pipes
- **Builtins in pipelines**: May need special handling
- **Pipeline exit status**: Need to ensure correct last command status

### 7. Redirection Implementation
- **Basic redirections**: `>`, `>>`, `<` are implemented in `redirection.rs`
- **Advanced redirections missing**: `>&`, `<&`, `>&-`, file descriptor duplication
- **Here documents**: Not implemented
- **Process substitution**: Not implemented

### 8. Arithmetic Expansion
- **`$((...))` syntax implemented**: Arithmetic evaluator exists in `arithmetic.rs`
- **Comprehensive operator support**: Most arithmetic operators are implemented
- **Variable references in arithmetic**: Supported
- **Assignment operators**: Partially implemented but needs testing

### 9. Parameter Expansion System
- **Basic `${parameter}` syntax**: Implemented in `param_expand.rs`
- **Advanced forms partially implemented**: 
  - `${parameter:-word}` - Use default value
  - `${parameter:=word}` - Assign default value  
  - `${parameter:?word}` - Display error if null/unset
  - `${parameter:+word}` - Use alternate value
  - `${#parameter}` - String length
  - `${parameter%pattern}` - Remove suffix pattern
  - `${parameter%%pattern}` - Remove largest suffix pattern
  - `${parameter#pattern}` - Remove prefix pattern
  - `${parameter##pattern}` - Remove largest prefix pattern
  - `${parameter/pattern/replacement}` - Pattern substitution
- **Case modification**: `${parameter^}`, `${parameter^^}`, `${parameter,}`, `${parameter,,}` implemented
- **Array support**: Not implemented

### 10. Built-in Commands Analysis
**✅ Implemented in rs-dash**:
- `cd` - Basic implementation
- `echo` - Basic implementation
- `exit` - Working
- `true`/`false` - Working
- `pwd` - Basic implementation
- `help` - Basic help display

**❌ Missing from rs-dash (compared to dash)**:

#### POSIX Special Builtins (must be built-in):
- `.` (dot) - Execute commands from file
- `:` (colon) - Null command  
- `break` - Exit from loops (needs loop support)
- `continue` - Continue loop iteration (needs loop support)
- `eval` - Evaluate arguments as shell commands
- `exec` - Replace shell with command
- `export` - Set export attribute for variables
- `readonly` - Make variables readonly
- `return` - Return from function (needs function support)
- `set` - Set shell options
- `shift` - Shift positional parameters
- `times` - Print process times
- `trap` - Handle signals
- `unset` - Unset variables

#### POSIX Standard Utilities (usually built-in):
- `alias`/`unalias` - Command aliasing
- `bg` - Background job control (needs job control)
- `command` - Execute simple command
- `fc` - Command history editor (needs history)
- `fg` - Foreground job control (needs job control)
- `getopts` - Parse positional parameters
- `hash` - Remember command locations
- `jobs` - List active jobs (needs job control)
- `kill` - Send signals to processes
- `read` - Read input
- `test`/`[` - Test conditions
- `type` - Describe command type
- `umask` - Set file creation mask
- `ulimit` - Control resource limits
- `wait` - Wait for process completion

### 11. Control Structures
- **Compound commands missing entirely**:
  - `if-then-elif-else-fi`
  - `for variable in words; do commands; done`
  - `while condition; do commands; done`
  - `until condition; do commands; done`
  - `case word in pattern) commands;; esac`
  - `select var in words; do commands; done`
- **Functions**: `name() { ... }` syntax not implemented
- **Subshells**: `(command)` not implemented

### 12. Job Control
- **Not implemented**: No job control infrastructure
- **Background execution**: `&` operator not supported
- **Process groups**: Not implemented
- **Signal handling**: Not implemented

### 13. History Features
- **Not implemented**: No command history
- **History expansion**: `!`, `!!`, etc. not supported
- **History file**: No persistence

### 14. Shell Options
- **Not implemented**: No `set` options support
- **Interactive options**: No special interactive features
- **Debug mode**: No `set -x` equivalent

### 15. Completion
- **Not implemented**: No tab completion

## Code Quality Issues

### 1. Dead Code Warnings
- **Multiple unused modules**: `grammar.rs`, `tokens.rs` contain unused code
- **Unused structs and enums**: `ASTNode`, `Redirect`, `Grammar`, `Token`, etc.
- **Build warnings**: 10 warnings during compilation

### 2. Architecture Issues
- **Incomplete module separation**: Some features spread across multiple files
- **Missing error handling**: Some panics may still exist
- **Limited test coverage**: Need more comprehensive tests

### 3. Performance Considerations
- **String copying**: May have unnecessary allocations
- **No command hashing**: PATH searched for each command
- **Simple data structures**: Could be optimized

## Testing Status

### ✅ Working Features (Based on Quick Tests)
1. Basic echo command
2. Exit status handling (`$?`)
3. Command substitution (`$(command)`)
4. Pipeline basics (`|`)
5. Command separators (`;`, `&&`, `||`)
6. Variable assignment and expansion
7. Basic builtins (`true`, `false`, `exit`, `pwd`, `cd`, `help`)

### ⚠️ Needs More Testing
1. Parameter expansion `${...}` forms
2. Arithmetic expansion `$((...))`
3. Redirections (`>`, `>>`, `<`)
4. Positional parameters (`$1`, `$2`, etc.)
5. Cross-platform behavior

### ❌ Known Broken/Missing
1. Control structures (if, for, while, etc.)
2. Functions
3. Job control
4. History
5. Advanced redirections
6. Here documents
7. Shell options
8. Signal handling
9. Most builtin commands

## Recommendations for Next Steps

### Phase 1: Fix Immediate Issues
1. Remove dead code or implement missing features
2. Add comprehensive test suite
3. Fix any remaining parsing issues

### Phase 2: Core POSIX Compliance
1. Implement control structures
2. Add remaining POSIX special builtins
3. Complete parameter expansion system
4. Implement functions

### Phase 3: Advanced Features
1. Job control implementation
2. History system
3. Signal handling
4. Shell options

### Phase 4: Polish & Optimization
1. Performance optimization
2. Enhanced error messages
3. Better cross-platform support
4. Documentation

## Comparison with Original Dash Architecture

### Parser Differences
- **dash**: Yacc-based parser with complete POSIX grammar
- **rs-dash**: Hand-written recursive descent with limited grammar

### Execution Model
- **dash**: Fork-exec model with full job control
- **rs-dash**: Simplified process spawning, no job control

### Variable System
- **dash**: Complex variable table with attributes (readonly, export, integer, array)
- **rs-dash**: Simple HashMap<String, String> with basic expansion

### Built-in Commands
- **dash**: Table-driven dispatch with function pointers
- **rs-dash**: Match statement with hardcoded functions

### Memory Management
- **dash**: Custom allocator with arenas for performance
- **rs-dash**: Rust ownership model (safer but different approach)

## Conclusion

rs-dash has a solid foundation with working basic shell functionality. The implementation shows good understanding of shell concepts and has several advanced features already implemented (parameter expansion, arithmetic expansion).

The main gaps are in:
1. Control structures and scripting features
2. Job control and process management
3. Comprehensive builtin command set
4. POSIX compliance for edge cases

The Rust implementation provides memory safety advantages and the modular architecture should facilitate adding missing features. The project is at a good stage for continued development toward full dash compatibility.