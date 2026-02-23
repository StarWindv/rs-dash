#!/usr/bin/env python3
"""
Test pipeline functionality
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

def test_pipeline():
    """Test pipeline functionality"""
    print("Testing pipeline functionality")
    print("=" * 60)
    
    tests = [
        # (command, expected_output, expected_exit_code, description)
        ("echo hello | echo world", "world", 0, "Simple pipe with echo"),
        ("echo test | grep test", "", 0, "Pipe with grep (may fail if grep not found)"),
        ("echo line1 && echo line2 | echo line3", "line3", 0, "Pipe with &&"),
        ("false || echo fallback | echo piped", "piped", 0, "Pipe with ||"),
    ]
    
    passed = 0
    failed = 0
    
    for cmd, expected_output, expected_code, desc in tests:
        print(f"\nTest: {desc}")
        print(f"Command: {cmd}")
        
        output, stderr, code = run_rs_dash(cmd)
        
        # For commands that may fail due to missing external commands,
        # we check if the error is about command not found
        if code != expected_code and "command not found" in stderr:
            print(f"SKIP (external command not available)")
            continue
            
        success = True
        if code != expected_code:
            print(f"FAIL: Exit code {code} (expected {expected_code})")
            success = False
        
        if output != expected_output:
            print(f"FAIL: Output '{output}' (expected '{expected_output}')")
            success = False
        
        if stderr:
            print(f"Stderr: {stderr}")
        
        if success:
            print(f"PASS")
            passed += 1
        else:
            failed += 1
    
    print(f"\n{'='*60}")
    print(f"Pipeline tests: {passed} passed, {failed} failed")
    
    return failed == 0

if __name__ == "__main__":
    success = test_pipeline()
    sys.exit(0 if success else 1)