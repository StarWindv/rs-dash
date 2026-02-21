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
4. Test suites in `test/` directory## Current Implementation Issues (Based on Testing)

### 1. Command Substitution Problems
- **Command substitution `$(pwd)` returns empty string**: The expansion seems to execute but doesn't capture output properly
- **Nested substitutions**: Not tested but likely broken

### 2. Variable Expansion Issues
- **`$?` returns "True"/"False" instead of numeric exit code**: Type conversion issue
- **Variable assignment doesn't affect following commands**: `MYVAR=hello; echo $MYVAR` doesn't expand
- **Variable scope problems**: Likely issues with environment inheritance

### 3. Parser Crashes
- **Panic in parser.rs line 91**: `called `Option::unwrap()` on a `None` value`
- **This happens with command substitution**: `echo $(pwd)` causes panic
- **Need better error handling**: Should handle parsing errors gracefully

### 4. External Command Issues
- **Windows-specific problems**: `cmd /c` not found, PATH searching issues
- **Cross-platform inconsistencies**: Different behavior on Windows vs Linux
- **Command not found handling**: Basic but could be improved

### 5. Exit Status Problems
- **Exit code 42 returns "False"**: PowerShell interprets non-zero as boolean false
- **Exit code propagation**: Need to ensure proper exit code handling
- **Builtin exit status**: `true` and `false` builtins work but status display is wrong

### 6. Pipeline Implementation
- **Simplified pipeline**: Not true parallel execution with pipes
- **Intermediate command output**: Not properly passed between commands
- **Builtins in pipelines**: Special handling needed

### 7. Redirection Issues
- **Test showed redirections as arguments**: `echo line1 > test.txt` treats `> test.txt` as arguments
- **File creation problems**: Windows file system issues
- **Append mode**: Implementation may have issues

## Specific Bugs Found

1. **Parser panic with command substitution**: Need to fix `unwrap()` on None
2. **`$?` displays boolean instead of number**: Type issue in expansion
3. **Variable assignment not expanding**: `$VAR` not working after assignment
4. **Windows PATH issues**: `cmd` not found despite being system command
5. **Exit code display**: PowerShell interprets exit codes as booleans

## Immediate Fixes Needed

### High Priority (Blocking Issues)
1. Fix parser panic in `src/modules/parser.rs:91`
2. Fix `$?` expansion to return numeric exit code
3. Fix variable expansion after assignment

### Medium Priority (Functional Issues)
1. Improve Windows command finding
2. Fix command substitution output capture
3. Improve redirection parsing

### Low Priority (Usability Issues)
1. Better error messages
2. More consistent cross-platform behavior
3. Enhanced help text