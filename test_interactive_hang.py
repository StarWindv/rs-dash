#!/usr/bin/env python3
"""
Test interactive pipeline issue described by user
"""

import subprocess
import os
import time
import threading

def test_interactive_hang():
    """Test if interactive shell hangs with pipeline"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print("Starting interactive rs-dash...")
    
    # Start interactive shell
    proc = subprocess.Popen([rs_dash_path],
                          stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE,
                          text=True,
                          bufsize=1,
                          universal_newlines=True)
    
    # Read initial output
    time.sleep(0.1)
    initial_output = ""
    try:
        # Try to read initial prompt
        while True:
            chunk = proc.stdout.read(1)
            if not chunk:
                break
            initial_output += chunk
            if "$ " in initial_output:
                break
    except:
        pass
    
    print(f"Initial shell output: {initial_output}")
    
    # Test 1: Simple command
    print("\nTest 1: Simple echo command")
    proc.stdin.write("echo hello\n")
    proc.stdin.flush()
    time.sleep(0.1)
    
    output = ""
    try:
        while True:
            chunk = proc.stdout.read(1)
            if not chunk:
                break
            output += chunk
            if "$ " in output:
                break
    except:
        pass
    
    print(f"Output: {output}")
    
    # Test 2: Pipeline command (user reported this hangs)
    print("\nTest 2: Pipeline command (echo hello | echo world)")
    proc.stdin.write("echo hello | echo world\n")
    proc.stdin.flush()
    
    # Wait for output with timeout
    start_time = time.time()
    output = ""
    hung = False
    
    try:
        while time.time() - start_time < 3:  # 3 second timeout
            chunk = proc.stdout.read(1)
            if chunk:
                output += chunk
                if "$ " in output:
                    break
            else:
                time.sleep(0.01)
    except:
        pass
    
    if "$ " not in output:
        print("ERROR: Shell appears to be hung! No prompt returned.")
        hung = True
    else:
        print(f"Output: {output}")
    
    # Test 3: Another command to see if shell is still responsive
    if not hung:
        print("\nTest 3: Another command after pipeline")
        proc.stdin.write("pwd\n")
        proc.stdin.flush()
        
        output = ""
        start_time = time.time()
        try:
            while time.time() - start_time < 2:
                chunk = proc.stdout.read(1)
                if chunk:
                    output += chunk
                    if "$ " in output:
                        break
                else:
                    time.sleep(0.01)
        except:
            pass
        
        if "$ " in output:
            print(f"Output: {output}")
            print("SUCCESS: Shell is still responsive")
        else:
            print("ERROR: Shell hung after pipeline command")
    
    # Clean up
    try:
        proc.stdin.write("exit\n")
        proc.stdin.flush()
        proc.wait(timeout=1)
    except:
        proc.kill()

if __name__ == "__main__":
    print("Testing interactive pipeline hanging issue")
    test_interactive_hang()