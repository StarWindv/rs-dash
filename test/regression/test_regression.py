#!/usr/bin/env python3
"""
Regression tests for rs-dash
Tests for specific issues that were fixed
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

def test_issue_exit_status():
    """Test exit status variable $? (Issue #1)"""
    print("=== Testing Exit Status Issue ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Original issue: echo $? not working
        ("exit status after true", "true; echo $?", "0", 0),
        ("exit status after false", "false; echo $?", "1", 0),
        
        # Chain of commands
        ("exit status chain 1", "true; false; echo $?", "1", 0),
        ("exit status chain 2", "false; true; echo $?", "0", 0),
        
        # With separators
        ("exit status with &&", "false && true; echo $?", "1", 0),
        ("exit status with ||", "false || true; echo $?", "0", 0),
        
        # In command substitution
        ("exit status in sub", 'echo "Status: $(false; echo $?)"', "Status: 1", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_issue_command_substitution():
    """Test command substitution parsing (Issue #2)"""
    print("\n=== Testing Command Substitution Issue ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Original issue: echo $(echo test) output "test)"
        ("basic command sub", "echo $(echo test)", "test", 0),
        ("command sub with args", "echo $(echo hello world)", "hello world", 0),
        
        # Multiple command substitutions
        ("multiple subs", "echo $(echo first) $(echo second)", "first second", 0),
        
        # Nested command substitutions
        ("nested sub", "echo $(echo $(echo nested))", "nested", 0),
        ("deep nested", "echo $(echo $(echo $(echo deep)))", "deep", 0),
        
        # Command substitution in middle of string
        ("sub in middle", "echo prefix$(echo middle)suffix", "prefixmiddlesuffix", 0),
        
        # Complex nested with variables
        ("complex nested", 'VAR=outer && echo $(echo $(echo $VAR))', "outer", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_issue_pipeline_hang():
    """Test pipeline hanging issue (Issue #3)"""
    print("\n=== Testing Pipeline Hang Issue ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Original issue: echo hello | grep ll hangs
        ("simple pipe no hang", "echo hello | grep ll", "hello", 0),
        ("pipe with cat", "echo test | cat", "test", 0),
        
        # Multiple pipes
        ("pipe chain", "echo hello | cat | grep ll", "hello", 0),
        
        # Pipe with builtins
        ("pipe builtins", "echo pipe | echo", "", 0),
        
        # Pipe in command substitution
        ("pipe in sub", "echo $(echo test | cat)", "test", 0),
        
        # Long pipeline (stress test)
        ("long pipe", "echo test | cat | cat | cat | cat", "test", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        # Use longer timeout for pipeline tests
        if runner.run_test(name, cmd, exit_code, expected, timeout=10):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_issue_variable_expansion():
    """Test various variable expansion edge cases"""
    print("\n=== Testing Variable Expansion Edge Cases ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Special variables
        ("shell pid", "echo $$", None, 0),  # Should output a number
        ("shell name", "echo $0", "rs-dash", 0),
        
        # Undefined variables
        ("undefined var", "echo $UNDEFINED", "", 0),
        
        # Variable with special characters
        ("var special chars", 'VAR="hello world" && echo $VAR', "hello world", 0),
        
        # Multiple $ expansions
        ("multiple $$", "echo $$ $$", None, 0),  # Two PIDs
        
        # $? in arithmetic context
        ("exit status reuse", "false; echo $?; true; echo $?", "1\n0", 0),
        
        # Variable in command substitution
        ("var in sub", 'MSG=hello && echo $(echo $MSG world)', "hello world", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_issue_parsing_edge_cases():
    """Test parsing edge cases"""
    print("\n=== Testing Parsing Edge Cases ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Empty commands
        ("empty command", "", "", 0),
        ("multiple semicolons", ";;;", "", 0),
        
        # Whitespace handling
        ("extra spaces", "  echo   test  ", "test", 0),
        ("tabs", "\techo\ttest\t", "test", 0),
        
        # Quotes in command substitution
        ("quotes in sub", 'echo "$(echo test)"', "test", 0),
        ("nested quotes", "echo '$(echo test)'", "$(echo test)", 0),
        
        # Special characters
        ("special chars", "echo 'test$test'", "test$test", 0),
        ("escape chars", 'echo "test\ntest"', "test\ntest", 0),
        
        # Command ending with separator
        ("trailing semicolon", "echo test;", "test", 0),
        ("trailing pipe", "echo test |", "", 0),  # Incomplete pipe
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def main():
    """Run all regression tests"""
    print("Running rs-dash Regression Tests")
    print("=" * 50)
    print("Testing specific issues that were previously fixed")
    print("=" * 50)
    
    total_passed = 0
    total_failed = 0
    
    # Run test suites
    passed, failed = test_issue_exit_status()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_issue_command_substitution()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_issue_pipeline_hang()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_issue_variable_expansion()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_issue_parsing_edge_cases()
    total_passed += passed
    total_failed += failed
    
    # Summary
    print("\n" + "=" * 50)
    print("REGRESSION TESTS SUMMARY")
    print("=" * 50)
    print(f"Total tests: {total_passed + total_failed}")
    print(f"Passed: {total_passed}")
    print(f"Failed: {total_failed}")
    
    if total_failed > 0:
        print("\nSome regression tests failed!")
        print("This means previously fixed issues may have regressed.")
        sys.exit(1)
    else:
        print("\nAll regression tests passed!")
        print("All known issues remain fixed.")
        sys.exit(0)

if __name__ == "__main__":
    main()