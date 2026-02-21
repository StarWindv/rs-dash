#!/usr/bin/env python3
"""
Test pipeline hanging issue
"""

import subprocess
import os
import threading
import time

def test_pipeline_hang():
    """Test if pipeline hangs"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print("Testing: echo hello | echo world")
    
    # Run with timeout
    try:
        result = subprocess.run([rs_dash_path, "-c", "echo hello | echo world"],
                               capture_output=True, text=True, timeout=2)
        print(f"Exit code: {result.returncode}")
        print(f"Stdout: {result.stdout}")
        print(f"Stderr: {result.stderr}")
        print("✓ Pipeline did not hang")
    except subprocess.TimeoutExpired:
        print("✗ Pipeline hung (timeout after 2 seconds)")
    
    print("\nTesting: echo test | echo test2")
    try:
        result = subprocess.run([rs_dash_path, "-c", "echo test | echo test2"],
                               capture_output=True, text=True, timeout=2)
        print(f"Exit code: {result.returncode}")
        print(f"Stdout: {result.stdout}")
        print(f"Stderr: {result.stderr}")
        print("✓ Pipeline did not hang")
    except subprocess.TimeoutExpired:
        print("✗ Pipeline hung (timeout after 2 seconds)")

def test_interactive_pipeline():
    """Test interactive mode with pipeline"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print("\n=== Testing interactive mode ===")
    
    # Start interactive shell
    proc = subprocess.Popen([rs_dash_path],
                          stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE,
                          text=True)
    
    # Send a command with pipeline
    try:
        stdout, stderr = proc.communicate("echo hello | echo world\n", timeout=2)
        print(f"Output: {stdout}")
        print(f"Error: {stderr}")
        print("✓ Interactive mode worked")
    except subprocess.TimeoutExpired:
        print("✗ Interactive mode hung")
        proc.kill()
        stdout, stderr = proc.communicate()
        print(f"Partial output: {stdout}")
        print(f"Partial error: {stderr}")

if __name__ == "__main__":
    print("Testing pipeline hanging issue")
    test_pipeline_hang()
    test_interactive_pipeline()