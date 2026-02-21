#!/usr/bin/env python3
"""
Test script for rs-dash
Compares behavior with native dash (if available)
"""

import os
import subprocess
import sys
import tempfile

def run_command(shell, cmd):
    """Run a command in the specified shell"""
    if shell == "dash":
        # Use native dash
        return subprocess.run(["dash", "-c", cmd], 
                            capture_output=True, text=True)
    elif shell == "rs-dash":
        # Use our rs-dash
        rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
        if os.name == 'nt':
            rs_dash_path += ".exe"
        return subprocess.run([rs_dash_path, "-c", cmd],
                            capture_output=True, text=True)
    else:
        raise ValueError(f"Unknown shell: {shell}")

def compare_output(test_name, cmd):
    """Compare output between dash and rs-dash"""
    print(f"\n=== Test: {test_name} ===")
    print(f"Command: {cmd}")
    
    # Try to run with dash
    dash_result = None
    try:
        dash_result = run_command("dash", cmd)
    except FileNotFoundError:
        print("dash not found, skipping comparison")
    
    # Run with rs-dash
    rs_dash_result = run_command("rs-dash", cmd)
    
    if dash_result:
        print(f"dash exit code: {dash_result.returncode}")
        print(f"dash stdout: {dash_result.stdout.strip()}")
        if dash_result.stderr:
            print(f"dash stderr: {dash_result.stderr.strip()}")
    
    print(f"rs-dash exit code: {rs_dash_result.returncode}")
    print(f"rs-dash stdout: {rs_dash_result.stdout.strip()}")
    if rs_dash_result.stderr:
        print(f"rs-dash stderr: {rs_dash_result.stderr.strip()}")
    
    if dash_result:
        if (dash_result.returncode == rs_dash_result.returncode and
            dash_result.stdout == rs_dash_result.stdout and
            dash_result.stderr == rs_dash_result.stderr):
            print("✓ PASS: Output matches")
            return True
        else:
            print("✗ FAIL: Output differs")
            return False
    else:
        print("? INFO: No comparison (dash not available)")
        return None

def main():
    print("Testing rs-dash against dash (where available)")
    
    tests = [
        # Basic commands
        ("echo basic", "echo hello world"),
        ("echo multiple args", "echo arg1 arg2 arg3"),
        ("echo with quotes", "echo 'hello world'"),
        ("echo with double quotes", 'echo "hello world"'),
        
        # Command separators
        ("semicolon", "echo first; echo second"),
        ("and operator success", "true && echo success"),
        ("and operator failure", "false && echo should not print"),
        ("or operator failure", "false || echo should print"),
        ("or operator success", "true || echo should not print"),
        
        # Builtins
        ("pwd", "pwd"),
        ("cd and pwd", "cd .. && pwd && cd -"),
        ("true command", "true"),
        ("false command", "false"),
        ("exit with code", "exit 42"),
        
        # External commands
        ("external command", "echo external test"),
        
        # Variables (basic)
        ("variable assignment", "VAR=test && echo $VAR"),
        
        # Redirections (basic)
        ("output redirection", "echo test > test.txt && cat test.txt && rm test.txt"),
    ]
    
    passed = 0
    failed = 0
    skipped = 0
    
    for test_name, cmd in tests:
        try:
            result = compare_output(test_name, cmd)
            if result is True:
                passed += 1
            elif result is False:
                failed += 1
            else:
                skipped += 1
        except Exception as e:
            print(f"Error running test {test_name}: {e}")
            failed += 1
    
    print(f"\n=== Summary ===")
    print(f"Total tests: {len(tests)}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Skipped: {skipped}")
    
    if failed > 0:
        sys.exit(1)

if __name__ == "__main__":
    main()