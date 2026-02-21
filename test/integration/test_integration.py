#!/usr/bin/env python3
"""
Integration tests for rs-dash
Tests complex scenarios and combinations
"""

import sys
import os
import tempfile
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

def test_pipelines():
    """Test pipeline functionality"""
    print("=== Testing Pipelines ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Basic pipelines
        ("simple pipe", "echo hello | cat", "hello", 0),
        ("pipe with grep", "echo hello world | grep hello", "hello world", 0),
        ("pipe chain", "echo test | cat | cat", "test", 0),
        
        # Pipeline with builtins
        ("pipe builtin echo", "echo hello | echo", "", 0),  # echo reads from stdin but doesn't output
        ("pipe with exit status", "false | true", "", 0),  # Pipeline exit status is last command
        
        # Complex pipelines
        ("pipe and separator", "echo first | cat && echo second", "first\nsecond", 0),
        ("pipe in subcommand", "echo $(echo pipe | cat)", "pipe", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected, timeout=10):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_redirections():
    """Test input/output redirections"""
    print("\n=== Testing Redirections ===")
    
    runner = TestRunner(verbose=True)
    
    # Create temporary files
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as tmp_out:
        tmp_out_path = tmp_out.name
    
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as tmp_in:
        tmp_in.write("input data\n")
        tmp_in_path = tmp_in.name
    
    tests = [
        # Output redirection
        ("output redirect", f"echo test > {tmp_out_path}", "", 0),
        ("append redirect", f"echo more >> {tmp_out_path}", "", 0),
        
        # Input redirection
        ("input redirect", f"cat < {tmp_in_path}", "input data", 0),
        
        # Combined redirections
        ("both redirects", f"cat < {tmp_in_path} > {tmp_out_path}", "", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        success = runner.run_test(name, cmd, exit_code, expected)
        
        # Clean up and check file contents for output tests
        if ">" in cmd:
            try:
                with open(tmp_out_path, 'r') as f:
                    content = f.read()
                    if "test" not in content and "more" not in content:
                        print(f"FAIL: Output not written to file")
                        success = False
            except:
                print(f"FAIL: Could not read output file")
                success = False
        
        if success:
            passed += 1
        else:
            failed += 1
    
    # Cleanup
    try:
        os.unlink(tmp_out_path)
        os.unlink(tmp_in_path)
    except:
        pass
    
    return passed, failed

def test_external_commands():
    """Test external command execution"""
    print("\n=== Testing External Commands ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Common external commands (should be available on most systems)
        ("external echo", "echo external", "external", 0),
        
        # Command with arguments
        ("external with args", "echo arg1 arg2 arg3", "arg1 arg2 arg3", 0),
        
        # Command not found
        ("command not found", "nonexistentcommand123", "", 127),
        
        # Path resolution
        ("ls command", "echo ls test", "ls test", 0),  # Just echo, not actual ls
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_interactive_mode():
    """Test interactive shell features"""
    print("\n=== Testing Interactive Mode ===")
    
    runner = TestRunner(verbose=True)
    
    # Test interactive commands
    commands = [
        "echo interactive test",
        "pwd",
        "exit"
    ]
    
    expected_outputs = [
        "interactive test",
        "$ "  # Prompt should appear
    ]
    
    if runner.run_interactive_test(commands, expected_outputs):
        print("Interactive test passed")
        return 1, 0
    else:
        print("Interactive test failed")
        return 0, 1

def test_script_execution():
    """Test script file execution"""
    print("\n=== Testing Script Execution ===")
    
    runner = TestRunner(verbose=True)
    
    # Create a test script
    script_content = """#!/bin/sh
echo "Script line 1"
echo "Script line 2"
VAR=test
echo "Variable: $VAR"
exit 0
"""
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.sh', delete=False) as f:
        f.write(script_content)
        script_path = f.name
    
    # Test script execution
    cmd = script_path
    
    expected_output = "Script line 1\nScript line 2\nVariable: test"
    
    if runner.run_test("script execution", cmd, 0, expected_output):
        print("Script test passed")
        passed = 1
        failed = 0
    else:
        print("Script test failed")
        passed = 0
        failed = 1
    
    # Cleanup
    try:
        os.unlink(script_path)
    except:
        pass
    
    return passed, failed

def test_complex_scenarios():
    """Test complex real-world scenarios"""
    print("\n=== Testing Complex Scenarios ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Complex variable usage
        ("complex vars", 'A=1 B=2 && echo "A=$A B=$B" && echo "Sum: $(expr $A + $B)"', "A=1 B=2\nSum: 3", 0),
        
        # Nested substitutions and pipes
        ("nested pipe", "echo $(echo hello | cat) world", "hello world", 0),
        
        # Conditional execution with variables
        ("conditional", 'RESULT=$(false || echo fallback) && echo "Result: $RESULT"', "Result: fallback", 0),
        
        # Multi-line command
        ("multi-line", 'echo "line1"; echo "line2"; echo "line3"', "line1\nline2\nline3", 0),
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
    """Run all integration tests"""
    print("Running rs-dash Integration Tests")
    print("=" * 50)
    
    total_passed = 0
    total_failed = 0
    
    # Run test suites
    passed, failed = test_pipelines()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_redirections()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_external_commands()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_interactive_mode()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_script_execution()
    total_passed += passed
    total_failed += failed
    
    passed, failed = test_complex_scenarios()
    total_passed += passed
    total_failed += failed
    
    # Summary
    print("\n" + "=" * 50)
    print("INTEGRATION TESTS SUMMARY")
    print("=" * 50)
    print(f"Total tests: {total_passed + total_failed}")
    print(f"Passed: {total_passed}")
    print(f"Failed: {total_failed}")
    
    if total_failed > 0:
        print("\nSome integration tests failed!")
        sys.exit(1)
    else:
        print("\nAll integration tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main()