#!/usr/bin/env python3
"""
Final test to verify all fixes
"""

import subprocess
import os

def run_test(name, cmd):
    """Run a single test"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print(f"\n{name}:")
    print(f"  Command: {cmd}")
    
    try:
        result = subprocess.run([rs_dash_path, "-c", cmd],
                               capture_output=True, text=True, timeout=3)
        
        if result.returncode == 0:
            print(f"  OK Exit: {result.returncode}")
        else:
            print(f"  FAIL Exit: {result.returncode}")
        
        if result.stdout.strip():
            print(f"  Output: {result.stdout.strip()}")
        
        if result.stderr.strip():
            print(f"  Error: {result.stderr.strip()}")
            
    except subprocess.TimeoutExpired:
        print(f"  ✗ TIMEOUT: Command hung (this was the bug!)")

def main():
    print("=== Final Verification of All Fixes ===")
    
    # Original issues from user
    print("\n--- Original Issues ---")
    
    # Issue 1: Variable expansion
    run_test("Issue 1: $? variable", "true; echo $?; false; echo $?")
    
    # Issue 2: Command substitution  
    run_test("Issue 2: $(command) substitution", "echo $(echo test)")
    
    # Issue 3: Pipeline (user said it hangs)
    run_test("Issue 3: Pipeline echo hello | echo world", "echo hello | echo world")
    
    # Additional tests
    print("\n--- Additional Tests ---")
    
    run_test("Special var $$", "echo $$")
    run_test("Special var $0", "echo $0")
    run_test("Variable assignment", "MYVAR=test && echo $MYVAR")
    run_test("Multiple commands", "echo a; echo b; echo c")
    run_test("Logical AND", "true && echo success")
    run_test("Logical OR", "false || echo failure")
    run_test("Complex pipeline", "echo start && echo middle | echo end")
    run_test("Nested operators", "false || echo alt | echo result")

if __name__ == "__main__":
    main()