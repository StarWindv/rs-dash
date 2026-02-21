# rs-dash Test Suite

This directory contains the comprehensive test suite for rs-dash.

## Directory Structure

```
test/
├── test_utils.py          # Test utilities and runner class
├── run_all_tests.py       # Main test runner
├── quick_test.py          # Quick test of core functionality
├── unit/                  # Unit tests
│   └── test_basic.py      # Basic functionality tests
├── integration/           # Integration tests
│   └── test_integration.py # Complex scenario tests
├── regression/            # Regression tests
│   └── test_regression.py  # Tests for fixed issues
└── scripts/               # Test scripts
    ├── test.sh           # Linux/macOS test script
    └── test.bat          # Windows test script
```

## Test Categories

### 1. Unit Tests (`test/unit/`)
- **Purpose**: Test individual components in isolation
- **Coverage**: Basic commands, command separators, variables, command substitution
- **Files**: `test_basic.py`

### 2. Integration Tests (`test/integration/`)
- **Purpose**: Test interactions between components
- **Coverage**: Pipelines, redirections, external commands, interactive mode
- **Files**: `test_integration.py`

### 3. Regression Tests (`test/regression/`)
- **Purpose**: Ensure previously fixed issues don't regress
- **Coverage**: Exit status ($?), command substitution, pipeline hanging
- **Files**: `test_regression.py`

## Running Tests

### Quick Test
```bash
# Run quick test of core functionality
python test/quick_test.py
```

### Full Test Suite
```bash
# Run all tests
python test/run_all_tests.py

# Or use platform-specific scripts
./test/scripts/test.sh    # Linux/macOS
test\scripts\test.bat     # Windows
```

### Individual Test Suites
```bash
# Run specific test suites
python test/unit/test_basic.py
python test/integration/test_integration.py
python test/regression/test_regression.py
```

## Test Utilities

The `TestRunner` class in `test_utils.py` provides:

1. **Command Execution**: Run commands in rs-dash or native dash
2. **Comparison Testing**: Compare rs-dash output with native dash
3. **Interactive Testing**: Test interactive shell mode
4. **Script Testing**: Test script file execution
5. **Timeout Handling**: Detect and handle hanging commands

## Adding New Tests

### 1. Unit Tests
Add test cases to `test/unit/test_basic.py` in the appropriate test function.

### 2. Integration Tests
Add complex scenarios to `test/integration/test_integration.py`.

### 3. Regression Tests
When fixing a bug, add a test case to `test/regression/test_regression.py` to prevent regression.

### Example Test Case
```python
def test_new_feature():
    runner = TestRunner(verbose=True)
    
    tests = [
        ("test name", "command", "expected output", expected_exit_code),
    ]
    
    for name, cmd, expected, exit_code in tests:
        runner.run_test(name, cmd, exit_code, expected)
```

## Test Coverage

The test suite covers:

### Core Features
- [x] Basic commands (echo, true, false, exit)
- [x] Command separators (;, &&, ||)
- [x] Variables ($VAR, $?, $$, $0)
- [x] Command substitution ($(command))
- [x] Pipelines (|)
- [x] Redirections (>, <, >>)
- [x] Interactive mode
- [x] Script execution

### Edge Cases
- [x] Empty commands
- [x] Extra whitespace
- [x] Special characters
- [x] Nested command substitution
- [x] Complex variable usage
- [x] Command not found errors

### Previously Fixed Issues
- [x] Exit status variable $?
- [x] Command substitution parsing
- [x] Pipeline hanging
- [x] Variable expansion edge cases

## Continuous Integration

Consider adding these tests to your CI pipeline:

```yaml
# Example GitHub Actions workflow
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      run: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    
    - name: Build
      run: cargo build
    
    - name: Run Tests
      run: python test/run_all_tests.py
```

## Notes

1. **Native Dash Comparison**: Tests compare with native dash when available
2. **Timeout Protection**: Tests have timeout protection to catch hanging commands
3. **Cross-Platform**: Tests work on Linux, macOS, and Windows
4. **Verbose Output**: Use `TestRunner(verbose=True)` for detailed output
5. **Error Handling**: Tests handle command not found, timeouts, and other errors gracefully