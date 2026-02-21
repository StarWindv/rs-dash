#!/usr/bin/env python3
"""
Basic unit tests for rs-dash
Tests fundamental functionality
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

def test_basic_commands():
    """Test basic built-in commands"""
    print("=== Testing Basic Commands ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Basic echo
        ("echo basic", "echo hello", "hello", 0),
        ("echo multiple", "echo hello world", "hello world", 0),
        ("echo empty", "echo", "", 0),
        
        # True and false
        ("true command", "true", "", 0),
        ("false command", "false", "", 1),
        
        # Exit
        ("exit zero", "exit 0", "", 0),
        ("exit non-zero", "exit 42", "", 42),
        ("exit no arg", "exit", "", 0),  # Should use last exit status
        
        # PWD
        ("pwd command", "pwd", None, 0),  # Output depends on current dir
        
        # Help
        ("help command", "help", None, 0),  # Just check it runs
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_command_separators():
    """Test command separators: ; && ||"""
    print("\n=== Testing Command Separators ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Semicolon
        ("semicolon basic", "echo first; echo second", "first\nsecond", 0),
        ("semicolon with exit", "echo test; exit 5", "test", 5),
        
        # AND operator
        ("and success", "true && echo success", "success", 0),
        ("and failure", "false && echo no", "", 1),
        ("and chain", "true && true && echo chain", "chain", 0),
        
        # OR operator
        ("or failure", "false || echo yes", "yes", 0),
        ("or success", "true || echo no", "", 0),
        ("or chain", "false || false || echo chain", "chain", 0),
        
        # Mixed operators
        ("mixed 1", "false || true && echo mixed", "mixed", 0),
        ("mixed 2", "true && false || echo fallback", "fallback", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_variables():
    """Test variable assignment and expansion"""
    print("\n=== Testing Variables ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Variable assignment
        ("var assign", "VAR=test", "", 0),
        ("var assign and use", "VAR=test && echo $VAR", "test", 0),
        
        # Special variables
        ("exit status true", "true; echo $?", "0", 0),
        ("exit status false", "false; echo $?", "1", 0),
        ("exit status chain", "false && true; echo $?", "1", 0),
        
        # Shell PID (should be a number)
        ("shell pid", "echo $$", None, 0),
        
        # Shell name
        ("shell name", "echo $0", "rs-dash", 0),
        
        # Multiple variables
        ("multiple vars", "A=1 B=2 && echo $A $B", "1 2", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_command_substitution():
    """Test command substitution $(command)"""
    print("\n=== Testing Command Substitution ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Basic command substitution
        ("basic sub", "echo $(echo test)", "test", 0),
        ("sub with args", "echo $(echo hello world)", "hello world", 0),
        
        # Substitution in arguments
        ("sub in middle", "echo prefix$(echo middle)suffix", "prefixmiddlesuffix", 0),
        ("multiple subs", "echo $(echo first) $(echo second)", "first second", 0),
        
        # Nested substitution (if supported)
        ("nested sub", "echo $(echo $(echo nested))", "nested", 0),
        
        # Substitution with variables
        ("sub with var", "MSG=hello && echo $(echo $MSG world)", "hello world", 0),
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
    """Run all unit tests"""
    print("Running rs-dash Unit Tests")
    print("=" * 50)
    
    total_passed = 0
    total_failed = 0
    
    # Run test suites
    passed, failed = test_basic_commands()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_command_separators()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_variables()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_command_substitution()
    total_passed += passed
    total_failed += failed
    
    # Summary
    print("\n" + "=" * 50)
    print("UNIT TESTS SUMMARY")
    print("=" * 50)
    print(f"Total tests: {total_passed + total_failed}")
    print(f"Passed: {total_passed}")
    print(f"Failed: {total_failed}")
    
    if total_failed > 0:
        print("\nSome tests failed!")
        sys.exit(1)
    else:
        print("\nAll unit tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main()