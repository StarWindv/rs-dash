#!/usr/bin/env python3
"""
Simple test for rs-dash
"""

import subprocess
import os
import tempfile

def run_test(name, command, expected_output="", expected_exit=0):
    """Run a test and check results"""
    print(f"\nTest: {name}")
    print(f"Command: {command}")
    
    # Run command
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    try:
        result = subprocess.run([rs_dash_path, "-c", command], 
                              capture_output=True, text=True, encoding='utf-8', errors='ignore')
    except Exception as e:
        print(f"Error running command: {e}")
        return False
    
    print(f"Exit code: {result.returncode} (expected: {expected_exit})")
    
    # Handle None output
    stdout = result.stdout or ""
    stderr = result.stderr or ""
    
    print(f"Output: {stdout.strip()}")
    if stderr.strip():
        print(f"Stderr: {stderr.strip()}")
    
    success = True
    if result.returncode != expected_exit:
        print(f"FAIL: Exit code mismatch")
        success = False
    
    if expected_output is not None:
        expected_clean = expected_output.strip() if expected_output else ""
        actual_clean = stdout.strip()
        if expected_clean != actual_clean:
            print(f"FAIL: Output mismatch")
            print(f"Expected: {expected_clean}")
            print(f"Got: {actual_clean}")
            success = False
    
    if success:
        print("PASS")
    
    return success

def main():
    print("Testing rs-dash implementation")
    
    tests = [
        # Basic echo
        ("echo basic", "echo hello", "hello", 0),
        ("echo multiple", "echo hello world", "hello world", 0),
        
        # Command separators
        ("semicolon", "echo first; echo second", "first\nsecond", 0),
        ("and success", "true && echo success", "success", 0),
        ("and failure", "false && echo no", "", 1),
        ("or failure", "false || echo yes", "yes", 0),
        ("or success", "true || echo no", "", 0),
        
        # Builtins
        ("true", "true", "", 0),
        ("false", "false", "", 1),
        ("exit code", "exit 42", "", 42),
        
        # External commands
        ("external echo", "echo external", "external", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if run_test(name, cmd, expected, exit_code):
            passed += 1
        else:
            failed += 1
    
    # Test interactive mode (simple)
    print("\n=== Testing interactive mode (simple) ===")
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    try:
        # Test with echo command via stdin
        result = subprocess.run([rs_dash_path], 
                              input="echo test\nexit\n", 
                              capture_output=True, text=True, encoding='utf-8')
        
        if "test" in result.stdout:
            print("Interactive test: PASS")
            passed += 1
        else:
            print(f"Interactive test: FAIL - Output: {result.stdout[:100]}...")
            failed += 1
    except Exception as e:
        print(f"Interactive test error: {e}")
        failed += 1
    
    print(f"\n=== Summary ===")
    print(f"Total tests: {len(tests) + 1}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    
    if failed > 0:
        exit(1)

if __name__ == "__main__":
    main()