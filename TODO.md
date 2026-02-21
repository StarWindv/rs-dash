# rs-dash TODO: Alignment with Standard Dash

## Project Goal
Complete implementation of a POSIX-compatible dash shell in Rust, matching all features of the original dash while maintaining Rust's safety guarantees.

## Current Version: 0.0.1
**Status**: Basic shell with limited functionality

## Priority 1: Core POSIX Compliance (Foundation)

### 1.1 Complete Parser Implementation
- [ ] Implement full POSIX shell grammar parser
- [ ] Add yacc/lex equivalent or hand-written recursive descent
- [ ] Support all token types from dash
- [ ] Handle complex quoting rules correctly
- [ ] Implement proper word splitting

**Files to modify**:
- `src/modules/parser.rs` - Complete rewrite or major extension
- New: `src/modules/grammar.rs` - Grammar definitions
- New: `src/modules/tokens.rs` - Token definitions

### 1.2 Parameter Expansion System
- [ ] Implement `${parameter}` syntax
- [ ] Add all expansion forms:
  - `${parameter:-word}` - Use default value
  - `${parameter:=word}` - Assign default value
  - `${parameter:?word}` - Display error if null/unset
  - `${parameter:+word}` - Use alternate value
- [ ] String operations:
  - `${#parameter}` - String length
  - `${parameter%word}` - Remove smallest suffix pattern
  - `${parameter%%word}` - Remove largest suffix pattern
  - `${parameter#word}` - Remove smallest prefix pattern
  - `${parameter##word}` - Remove largest prefix pattern
  - `${parameter/pattern/string}` - Pattern substitution
  - `${parameter//pattern/string}` - Global pattern substitution

**Files to modify**:
- `src/modules/expansion.rs` - Major extension
- New: `src/modules/param_expand.rs` - Parameter expansion engine

### 1.3 Arithmetic Expansion
- [ ] Implement `$((expression))` syntax
- [ ] Support all arithmetic operators:
  - Basic: `+`, `-`, `*`, `/`, `%`
  - Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
  - Logical: `!`, `&&`, `||`
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Ternary: `? :`
  - Comma: `,`
- [ ] Handle integer constants in different bases
- [ ] Support variable references in expressions

**Files to create**:
- `src/modules/arithmetic.rs` - Arithmetic parser and evaluator
- `src/modules/arith_lexer.rs` - Lexer for arithmetic expressions
- `src/modules/arith_parser.rs` - Parser for arithmetic expressions

### 1.4 Positional Parameters
- [ ] Implement `$1`, `$2`, ... `$9`, `${10}`, etc.
- [ ] Support `$@` and `$*` with proper quoting differences
- [ ] Implement `shift` builtin
- [ ] Handle `set --` for setting positional parameters

**Files to modify**:
- `src/modules/shell.rs` - Add positional parameters storage
- `src/modules/expansion.rs` - Add positional parameter expansion
- New: `src/modules/builtins/set.rs` - `set` builtin
- New: `src/modules/builtins/shift.rs` - `shift` builtin

### 1.5 Special Parameters
- [ ] Implement all special parameters:
  - `$?` - Exit status (already implemented)
  - `$$` - PID (already implemented)
  - `$!` - PID of last background command
  - `$0` - Shell name (already implemented)
  - `$-` - Current option flags
  - `$#` - Number of positional parameters

## Priority 2: Shell Scripting Features

### 2.1 Compound Commands
- [ ] **If statements**: `if-then-elif-else-fi`
- [ ] **For loops**: `for var in words; do commands; done`
- [ ] **While loops**: `while condition; do commands; done`
- [ ] **Until loops**: `until condition; do commands; done`
- [ ] **Case statements**: `case word in pattern) commands;; esac`
- [ ] **Select statements**: `select var in words; do commands; done`

**Files to create**:
- `src/modules/control.rs` - Control structure execution
- `src/modules/condition.rs` - Condition evaluation ([ command)
- New: `src/modules/builtins/test.rs` - `test` and `[` builtins

### 2.2 Functions
- [ ] Function definition: `name() compound-command`
- [ ] Local variables within functions
- [ ] `return` builtin for function exit
- [ ] Function name space management
- [ ] Function tracing support

**Files to create**:
- `src/modules/functions.rs` - Function storage and execution
- New: `src/modules/builtins/return.rs` - `return` builtin

### 2.3 Here Documents
- [ ] Basic here-doc: `<< EOF`
- [ ] Here-doc with tab suppression: `<<- EOF`
- [ ] Quoted here-doc: `<< "EOF"` (no expansion)
- [ ] Here-string: `<<< "string"`

**Files to modify**:
- `src/modules/redirection.rs` - Extend redirection handling
- New: `src/modules/heredoc.rs` - Here-document processing

## Priority 3: Built-in Commands

### 3.1 POSIX Special Builtins (Must implement)
- [ ] `.` (dot) - `src/modules/builtins/dot.rs`
- [ ] `:` (colon) - `src/modules/builtins/colon.rs`
- [ ] `break` - `src/modules/builtins/break.rs`
- [ ] `continue` - `src/modules/builtins/continue.rs`
- [ ] `eval` - `src/modules/builtins/eval.rs`
- [ ] `exec` - `src/modules/builtins/exec.rs`
- [ ] `export` - `src/modules/builtins/export.rs`
- [ ] `readonly` - `src/modules/builtins/readonly.rs`
- [ ] `set` - `src/modules/builtins/set.rs`
- [ ] `times` - `src/modules/builtins/times.rs`
- [ ] `trap` - `src/modules/builtins/trap.rs`
- [ ] `unset` - `src/modules/builtins/unset.rs`

### 3.2 POSIX Standard Utilities
- [ ] `alias`/`unalias` - `src/modules/builtins/alias.rs`
- [ ] `bg` - `src/modules/builtins/bg.rs` (requires job control)
- [ ] `command` - `src/modules/builtins/command.rs`
- [ ] `fc` - `src/modules/builtins/fc.rs` (requires history)
- [ ] `fg` - `src/modules/builtins/fg.rs` (requires job control)
- [ ] `getopts` - `src/modules/builtins/getopts.rs`
- [ ] `hash` - `src/modules/builtins/hash.rs`
- [ ] `jobs` - `src/modules/builtins/jobs.rs` (requires job control)
- [ ] `kill` - `src/modules/builtins/kill.rs`
- [ ] `read` - `src/modules/builtins/read.rs`
- [ ] `type` - `src/modules/builtins/type.rs`
- [ ] `umask` - `src/modules/builtins/umask.rs`
- [ ] `ulimit` - `src/modules/builtins/ulimit.rs`
- [ ] `wait` - `src/modules/builtins/wait.rs`

### 3.3 Enhanced Existing Builtins
- [ ] `cd`: Add `CDPATH` support
- [ ] `echo`: Add `-n`, `-e`, `-E` options
- [ ] `exit`: Handle exit traps
- [ ] `pwd`: Add `-L` and `-P` options

## Priority 4: Job Control & Process Management

### 4.1 Job Control Infrastructure
- [ ] Process group management
- [ ] Background execution with `&`
- [ ] Job table maintenance
- [ ] Terminal control (tcsetpgrp, etc.)
- [ ] Signal handling for job control

**Files to create**:
- `src/modules/jobs.rs` - Job table and management
- `src/modules/process.rs` - Process group handling
- `src/modules/signals.rs` - Signal handling

### 4.2 Job Control Builtins
- [ ] `jobs` - List jobs
- [ ] `fg` - Bring job to foreground
- [ ] `bg` - Continue job in background
- [ ] `wait` - Wait for job completion

### 4.3 Signal Handling
- [ ] `trap` builtin implementation
- [ ] Signal delivery to processes
- [ ] Signal masks and handlers
- [ ] Ignored signals handling

## Priority 5: Advanced Features

### 5.1 Arrays
- [ ] Indexed array support
- [ ] Associative arrays (if dash supports)
- [ ] Array expansion `${array[@]}` and `${array[*]}`
- [ ] Array slicing `${array[@]:start:length}`

### 5.2 History
- [ ] Command history storage
- [ ] History file management (`~/.dash_history`)
- [ ] History expansion (`!`, `!!`, `!string`, etc.)
- [ ] `fc` builtin for history editing

**Files to create**:
- `src/modules/history.rs` - History management
- `src/modules/histexpand.rs` - History expansion

### 5.3 Completion
- [ ] Basic tab completion
- [ ] Programmable completion framework
- [ ] Completion for builtins

### 5.4 Shell Options
- [ ] `set -o` options implementation
- [ ] Option inheritance for subshells
- [ ] Interactive mode options

## Priority 6: Performance & Optimization

### 6.1 Command Hashing
- [ ] `hash` builtin implementation
- [ ] Command path caching
- [ ] Hash table for fast command lookup

### 6.2 Builtin Optimization
- [ ] Fast path for simple commands
- [ ] Avoid fork for builtins when possible
- [ ] Optimized variable expansion

### 6.3 Memory Management
- [ ] String interning for common strings
- [ ] Arena allocation for parse trees
- [ ] Efficient environment variable storage

## Priority 7: Testing & Compliance

### 7.1 Test Suite
- [ ] Unit tests for all modules
- [ ] Integration tests matching dash test suite
- [ ] POSIX compliance tests
- [ ] Performance benchmarks

### 7.2 POSIX Compliance
- [ ] Pass POSIX test suite
- [ ] Implement all required features
- [ ] Document deviations from standard

### 7.3 Dash Compatibility
- [ ] Test compatibility with existing dash scripts
- [ ] Match dash behavior edge cases
- [ ] Document differences from dash

## Implementation Strategy

### Phase 1: Foundation
1. Complete parser implementation
2. Basic parameter expansion
3. Arithmetic expansion
4. Positional parameters

### Phase 2: Scripting
1. Compound commands
2. Functions
3. Here documents
4. Additional builtins

### Phase 3: Job Control
1. Job control infrastructure
2. Signal handling
3. Job control builtins

### Phase 4: Advanced Features
1. Arrays
2. History
3. Completion
4. Performance optimizations

### Phase 5: Polish & Testing
1. Comprehensive test suite
2. POSIX compliance
3. Performance benchmarking
4. Documentation

## File Structure Changes

### New Directories
```
src/modules/builtins/      # All builtin commands
src/modules/control/       # Control structures
src/modules/expansion/     # Expansion subsystems
src/modules/jobs/         # Job control
src/modules/history/      # Command history
src/modules/test/         # Test framework
```

### Modified Files
- `src/modules/parser.rs` - Complete rewrite
- `src/modules/expansion.rs` - Major extension
- `src/modules/shell.rs` - Enhanced with new features
- `src/main.rs` - Updated initialization

## Testing Approach

1. **Unit Tests**: Each module has comprehensive unit tests
2. **Integration Tests**: Test shell behavior as a whole
3. **Compatibility Tests**: Compare output with dash
4. **Performance Tests**: Benchmark against dash
5. **Fuzz Testing**: Random input testing

## Documentation

1. **User Manual**: Complete documentation of all features
2. **Developer Guide**: Architecture and extension guide
3. **API Documentation**: Rustdoc for all public APIs
4. **Migration Guide**: From dash to rs-dash

## Success Metrics

1. **Feature Parity**: 100% of dash features implemented
2. **POSIX Compliance**: Pass POSIX test suite
3. **Performance**: Comparable or better than dash
4. **Memory Safety**: Zero unsafe code where possible
5. **Code Quality**: High test coverage, clean architecture

## Risks & Mitigations

1. **Complexity Risk**: Break into manageable chunks
2. **Performance Risk**: Profile and optimize iteratively
3. **Compatibility Risk**: Test extensively with existing scripts
4. **Maintenance Risk**: Clean architecture, good documentation

## Contributing Guidelines

1. **Code Style**: Follow Rustfmt and Clippy
2. **Testing**: Write tests for new features
3. **Documentation**: Document public APIs
4. **Performance**: Profile changes affecting performance
5. **Compatibility**: Maintain dash compatibility

## References

1. `../c-dash/` - Original dashv0.5.3 source code
2. POSIX Shell Standard (IEEE Std 1003.1-2017)
3. dash man pages and documentation
4. Existing test suites in `test/` directory

---

*Last Updated: 2026-02-22*
*Version: TODO v0.0.1

