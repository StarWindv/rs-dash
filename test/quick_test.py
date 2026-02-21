#!/usr/bin/env python3
"""
Quick test for rs-dash
Run a subset of important tests quickly
"""

import sys
import os
sys.path.append(os.path.dirname(__file__))

from test_utils import TestRunner

def quick_test():
    """Run quick test of core functionality"""
    print("=== Quick Test - Core Functionality ===")
    
    runner = TestRunner(verbose=True)
    
    # Most important tests that must pass
    critical_tests = [
        # Basic echo
        ("echo", "echo hello world", "hello world", 0),
        
        # Exit status (was a major issue)
        ("exit status", "false; echo $?", "1", 0),
        
        # Command substitution (was a major issue)
        ("command sub", "echo $(echo test)", "test", 0),
        
        # Pipeline (was hanging) - use echo as it's available on all platforms
        ("pipeline", "echo test | echo", "", 0),
        
        # Command separators
        ("semicolon", "echo first; echo second", "first\nsecond", 0),
        ("and operator", "true && echo success", "success", 0),
        ("or operator", "false || echo fallback", "fallback", 0),
        
        # Variables
        ("variables", "VAR=value && echo $VAR", "value", 0),
        
        # Builtins
        ("true/false", "true && false || echo mixed", "mixed", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in critical_tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    # Summary
    print(f"\n{'='*50}")
    print("QUICK TEST SUMMARY")
    print(f"{'='*50}")
    print(f"Total tests: {passed + failed}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    
    if failed == 0:
        print("\nAll critical tests passed!")
        return 0
    else:
        print("\nSome critical tests failed!")
        return 1

if __name__ == "__main__":
    sys.exit(quick_test())