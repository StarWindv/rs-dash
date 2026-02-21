# rs-dash Test Suite

This directory contains comprehensive tests for rs-dash, organized by functionality.

## Test Organization

### 1. Unit Tests (`unit/`)
- Basic command parsing and execution
- Built-in command functionality
- Core shell features

### 2. Integration Tests (`integration/`)
- Complex command combinations
- Pipeline and redirection tests
- Interactive mode tests
- Script execution tests

### 3. Regression Tests (`regression/`)
- Tests for specific bugs that were fixed
- Ensure previously fixed issues don't regress

### 4. Conformance Tests (`conformance/`)
- POSIX shell standard compliance tests
- Comparison with native dash behavior
- Cross-platform compatibility tests

### 5. Performance Tests (`performance/`)
- Execution speed benchmarks
- Memory usage tests
- Stress tests for complex scripts

## Running Tests

```bash
# Run all tests
python run_all_tests.py

# Run specific test suite
python unit/test_basic.py
python integration/test_pipelines.py

# Run quick test
python quick_test.py
```

## Test Dependencies

- Python 3.6+
- rs-dash built (in target/debug/ or target/release/)
- Native dash (optional, for comparison tests)

## Test Philosophy

1. **Comprehensive**: Cover all shell features
2. **Accurate**: Match native dash behavior
3. **Reliable**: Tests should be deterministic
4. **Informative**: Clear error messages and diagnostics
5. **Maintainable**: Easy to add new tests

## Adding New Tests

1. Choose the appropriate test directory
2. Follow existing test patterns
3. Include clear test descriptions
4. Add comparison with native dash when possible
5. Ensure tests pass on all supported platforms