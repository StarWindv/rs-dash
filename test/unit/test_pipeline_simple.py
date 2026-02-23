#!/usr/bin/env python3
"""
Test pipeline data flow - simplified version
"""

import sys
import os
import subprocess

def run_rs_dash(cmd):
    """Run a command through rs-dash and return output and exit code"""
    target_dir = os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug")
    rs_dash_exe = "rs-dash.exe" if os.name == 'nt' else "rs-dash"
    rs_dash_path = os.path.join(target_dir, rs_dash_exe)
    
    result = subprocess.run([rs_dash_path, "-c", cmd], 
                          capture_output=True, text=True)
    
    return result.stdout.strip(), result.stderr.strip(), result.returncode

def test_simple_pipeline():
    """Test simple pipeline scenarios"""
    print("Testing simple pipeline scenarios")
    print("=" * 60)
    
    tests = [
        # Test 1: echo through multiple pipes
        ("echo hello | cat", "hello", 0, "echo to cat pipe"),
        
        # Test 2: multiple echos
        ("echo a && echo b | echo c", "c", 0, "multiple commands with pipe"),
        
        # Test 3: variable in pipe
        ("MSG=test && echo $MSG | cat", "test", 0, "variable in pipe"),
    ]
    
    passed = 0
    failed = 0
    
    for cmd, expected_output, expected_code, desc in tests:
        print(f"\nTest: {desc}")
        print(f"Command: {cmd}")
        
        output, stderr, code = run_rs_dash(cmd)
        
        success = True
        if code != expected_code:
            print(f"FAIL: Exit code {code} (expected {expected_code})")
            if stderr:
                print(f"Stderr: {stderr}")
            success = False
        
        if output != expected_output:
            print(f"FAIL: Output '{output}' (expected '{expected_output}')")
            success = False
        
        if success:
            print(f"PASS")
            passed += 1
        else:
            failed += 1
    
    print(f"\n{'='*60}")
    print(f"Simple pipeline tests: {passed} passed, {failed} failed")
    
    return failed == 0

if __name__ == "__main__":
    success = test_simple_pipeline()
    sys.exit(0 if success else 1)