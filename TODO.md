# rs-dash TODO: Alignment with Standard Dash

## Project Goal
Complete implementation of a POSIX-compatible dash shell in Rust, matching all features of the original dash while maintaining Rust's safety guarantees.

## Current Status: v0.0.1
**Based on Code Review**: Basic shell with core functionality working, but missing many POSIX features

## ✅ Currently Working Features
1. **Basic command execution**: External commands with PATH search
2. **Built-in commands**: `cd`, `echo`, `exit`, `true`/`false`, `pwd`, `help`
3. **Command separators**: `;`, `&&`, `||`
4. **Pipelines**: Basic `|` support
5. **Redirections**: `>`, `>>`, `<`
6. **Variables**: Assignment and basic expansion (`$VAR`)
7. **Command substitution**: `$(command)` syntax
8. **Parameter expansion**: Basic `${parameter}` forms
9. **Arithmetic expansion**: `$((expression))` with full operator support
10. **Positional parameters**: `$1`, `$2`, etc. basic support

## Immediate Issues to Address

### 1. Clean Up Dead Code
- **Remove or implement unused modules**: `grammar.rs`, `tokens.rs`
- **Fix compilation warnings**: 10 warnings currently
- **Consolidate expansion logic**: `expansion.rs`, `param_expand.rs`, `arithmetic.rs`

### 2. Improve Test Coverage
- **Expand unit tests**: Test all implemented features
- **Add integration tests**: Compare with dash behavior
- **Create conformance tests**: POSIX compliance testing

### 3. Fix Known Bugs
- **Command substitution edge cases**: Nested substitutions, quoting
- **Redirection parsing**: Ensure `>` and `<` are not treated as arguments
- **Cross-platform issues**: Windows PATH handling, line endings

## Priority 1: Core POSIX Compliance (Foundation)

### 1.1 Complete Parser Implementation
- [ ] **Fix current parser issues**: Handle complex quoting, nested expansions
- [ ] **Add full POSIX grammar**: Support all shell grammar constructs
- [ ] **Implement token stream**: Use `tokens.rs` and `grammar.rs` modules
- [ ] **Add parse tree generation**: Build proper AST for complex commands

**Files to modify**:
- `src/modules/parser.rs` - Major extension
- `src/modules/grammar.rs` - Implement unused code
- `src/modules/tokens.rs` - Implement unused code

### 1.2 Control Structures Implementation
- [ ] **If statements**: `if-then-elif-else-fi`
- [ ] **For loops**: `for var in words; do commands; done`
- [ ] **While loops**: `while condition; do commands; done`
- [ ] **Until loops**: `until condition; do commands; done`
- [ ] **Case statements**: `case word in pattern) commands;; esac`
- [ ] **Select statements**: `select var in words; do commands; done`

**Files to create**:
- `src/modules/control.rs` - Control structure execution
- `src/modules/condition.rs` - Condition evaluation
- New: `src/modules/builtins/test.rs` - `test` and `[` builtins

### 1.3 Functions
- [ ] **Function definition**: `name() compound-command`
- [ ] **Local variables**: Variable scoping within functions
- [ ] **`return` builtin**: Function exit with status
- [ ] **Function namespace**: Manage function definitions

**Files to create**:
- `src/modules/functions.rs` - Function storage and execution
- New: `src/modules/builtins/return.rs` - `return` builtin

### 1.4 Subshells and Process Substitution
- [ ] **Subshell execution**: `(command)` syntax
- [ ] **Process substitution**: `<(command)` and `>(command)`
- [ ] **Coprocesses**: `command |&` syntax

## Priority 2: Built-in Commands Completion

### 2.1 POSIX Special Builtins (Must implement)
- [ ] `.` (dot) - `src/modules/builtins/dot.rs`
- [ ] `:` (colon) - `src/modules/builtins/colon.rs`
- [ ] `break` - `src/modules/builtins/break.rs` (requires loops)
- [ ] `continue` - `src/modules/builtins/continue.rs` (requires loops)
- [ ] `eval` - `src/modules/builtins/eval.rs`
- [ ] `exec` - `src/modules/builtins/exec.rs`
- [ ] `export` - `src/modules/builtins/export.rs`
- [ ] `readonly` - `src/modules/builtins/readonly.rs`
- [ ] `set` - `src/modules/builtins/set.rs`
- [ ] `shift` - `src/modules/builtins/shift.rs`
- [ ] `times` - `src/modules/builtins/times.rs`
- [ ] `trap` - `src/modules/builtins/trap.rs`
- [ ] `unset` - `src/modules/builtins/unset.rs`

### 2.2 POSIX Standard Utilities
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
- [ ] `test`/`[` - `src/modules/builtins/test.rs`
- [ ] `type` - `src/modules/builtins/type.rs`
- [ ] `umask` - `src/modules/builtins/umask.rs`
- [ ] `ulimit` - `src/modules/builtins/ulimit.rs`
- [ ] `wait` - `src/modules/builtins/wait.rs`

### 2.3 Enhanced Existing Builtins
- [ ] `cd`: Add `CDPATH` support, `-L` and `-P` options
- [ ] `echo`: Add `-n`, `-e`, `-E` options
- [ ] `exit`: Handle exit traps, optional status argument
- [ ] `pwd`: Add `-L` and `-P` options

## Priority 3: Job Control & Process Management

### 3.1 Job Control Infrastructure
- [ ] **Process group management**: Setpgid, tcsetpgrp
- [ ] **Background execution**: `&` operator support
- [ ] **Job table**: Track background jobs
- [ ] **Signal handling**: SIGINT, SIGTSTP, SIGCONT

**Files to create**:
- `src/modules/jobs.rs` - Job table and management
- `src/modules/process.rs` - Process group handling
- `src/modules/signals.rs` - Signal handling

### 3.2 Job Control Builtins
- [ ] `jobs` - List background jobs
- [ ] `fg` - Bring job to foreground
- [ ] `bg` - Continue job in background
- [ ] `wait` - Wait for job completion

## Priority 4: Advanced Shell Features

### 4.1 Here Documents
- [ ] **Basic here-doc**: `<< EOF`
- [ ] **Here-doc with tab suppression**: `<<- EOF`
- [ ] **Quoted here-doc**: `<< "EOF"` (no expansion)
- [ ] **Here-string**: `<<< "string"`

**Files to modify**:
- `src/modules/redirection.rs` - Extend redirection handling
- New: `src/modules/heredoc.rs` - Here-document processing

### 4.2 Arrays
- [ ] **Indexed arrays**: `array=(element1 element2)`
- [ ] **Array expansion**: `${array[@]}`, `${array[*]}`
- [ ] **Array slicing**: `${array[@]:start:length}`
- [ ] **Associative arrays**: If dash supports them

### 4.3 History System
- [ ] **Command history storage**: In-memory and file-based
- [ ] **History expansion**: `!`, `!!`, `!n`, `!-n`, `!string`
- [ ] **`fc` builtin**: History editor
- [ ] **History file**: `~/.dash_history`

**Files to create**:
- `src/modules/history.rs` - History management
- `src/modules/histexpand.rs` - History expansion

### 4.4 Shell Options
- [ ] **`set` options**: `-e`, `-u`, `-x`, `-o option`
- [ ] **Option inheritance**: For subshells
- [ ] **Interactive options**: `-i`, `-m`, `-s`

## Priority 5: Performance & Optimization

### 5.1 Command Hashing
- [ ] **`hash` builtin implementation**
- [ ] **Command path caching**: Avoid repeated PATH searches
- [ ] **Hash table**: Fast command lookup

### 5.2 Builtin Optimization
- [ ] **Fast path for simple commands**: Avoid fork when possible
- [ ] **Builtin preference**: Builtins vs external commands
- [ ] **String interning**: For common strings

### 5.3 Memory Management
- [ ] **Arena allocation**: For parse trees
- [ ] **Efficient environment storage**: Reduce copying
- [ ] **String pooling**: For common variable names

## Priority 6: Testing & Compliance

### 6.1 Comprehensive Test Suite
- [ ] **Unit tests**: All modules at 80%+ coverage
- [ ] **Integration tests**: End-to-end shell behavior
- [ ] **Conformance tests**: POSIX test suite compatibility
- [ ] **Performance tests**: Benchmark against dash

### 6.2 POSIX Compliance
- [ ] **Pass POSIX test suite**: All required features
- [ ] **Document deviations**: From POSIX standard
- [ ] **Compatibility testing**: With existing dash scripts

### 6.3 Dash Compatibility
- [ ] **Behavior matching**: Edge cases and error messages
- [ ] **Bug-for-bug compatibility**: Where appropriate
- [ ] **Performance comparison**: Similar or better performance

## Implementation Strategy

### Phase 1: Foundation & Cleanup (Current)
1. Clean up dead code and warnings
2. Expand test coverage
3. Fix known bugs
4. Document current architecture

### Phase 2: Core Scripting Features
1. Implement control structures (if, for, while, case)
2. Add functions support
3. Complete builtin command set
4. Implement here documents

### Phase 3: Job Control & Advanced Features
1. Job control infrastructure
2. Signal handling
3. History system
4. Shell options

### Phase 4: Polish & Optimization
1. Performance optimization
2. Memory management improvements
3. Enhanced error messages
4. Better cross-platform support

### Phase 5: Compliance & Testing
1. POSIX compliance testing
2. Comprehensive test suite
3. Performance benchmarking
4. Documentation completion

## File Structure Changes

### Current Structure Issues
1. **Dead code**: `grammar.rs`, `tokens.rs` unused
2. **Builtin organization**: All builtins in `builtins.rs` - should be split
3. **Expansion logic**: Spread across multiple files

### Proposed New Structure
```
src/
├── main.rs
└── modules/
    ├── shell.rs              # Main shell structure
    ├── parser.rs             # Command parsing
    ├── grammar.rs            # Shell grammar (to be implemented)
    ├── tokens.rs             # Token definitions (to be implemented)
    ├── expansion/            # All expansion types
    │   ├── mod.rs
    │   ├── variable.rs       # $VAR expansion
    │   ├── param.rs          # ${...} expansion
    │   ├── arithmetic.rs     # $((...)) expansion
    │   └── command.rs        # $(...) expansion
    ├── builtins/             # Builtin commands
    │   ├── mod.rs
    │   ├── cd.rs
    │   ├── echo.rs
    │   └── ... (one file per builtin)
    ├── control/              # Control structures
    │   ├── mod.rs
    │   ├── if.rs
    │   ├── for.rs
    │   └── ...
    ├── jobs/                 # Job control
    │   ├── mod.rs
    │   ├── jobtable.rs
    │   └── signals.rs
    ├── redirection.rs        # Redirections
    ├── pipeline.rs           # Pipelines
    ├── functions.rs          # Functions
    ├── history.rs            # Command history
    └── options.rs            # Shell options
```

## Testing Approach

1. **Unit Tests**: Each module independently testable
2. **Integration Tests**: Full shell behavior testing
3. **Conformance Tests**: POSIX standard compliance
4. **Performance Tests**: Benchmarking against dash
5. **Fuzz Testing**: Random input testing for stability

## Success Metrics

1. **Feature Parity**: 100% of dash features implemented
2. **POSIX Compliance**: Pass POSIX test suite
3. **Performance**: Comparable or better than dash
4. **Memory Safety**: Minimal unsafe code
5. **Code Quality**: High test coverage, clean architecture

## Risks & Mitigations

1. **Complexity Risk**: Break into manageable chunks, focus on core features first
2. **Performance Risk**: Profile early and often, optimize bottlenecks
3. **Compatibility Risk**: Test with existing scripts, match dash behavior
4. **Maintenance Risk**: Clean architecture, good documentation, comprehensive tests

## References

1. `../c-dash/` - Original dash v0.5.3 source code
2. POSIX Shell Standard (IEEE Std 1003.1-2017)
3. dash man pages and documentation
4. Existing test suites in `test/` directory

---

*Last Updated: 2026-02-22 (Based on code review)*
*Version: TODO v0.0.2*