#!/usr/bin/env python3
"""
Test exact scenario described by user
"""

import subprocess
import os
import time

def test_user_scenario():
    """Test the exact scenario described by user"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print("Testing user's exact scenario:")
    print("1. echo hello | grep ll")
    print("2. Should output nothing (grep not found on Windows)")
    print("3. But shell should return to prompt")
    
    # Test non-interactive mode first
    print("\n=== Non-interactive mode ===")
    result = subprocess.run([rs_dash_path, "-c", "echo hello | grep ll"],
                           capture_output=True, text=True, timeout=5)
    print(f"Exit code: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    print(f"Stderr: {result.stderr}")
    
    if result.returncode == 127:
        print("Note: grep not found (expected on Windows)")
    
    # Test with a command that exists on Windows
    print("\n=== Testing with Windows command ===")
    result = subprocess.run([rs_dash_path, "-c", "echo hello | findstr hello"],
                           capture_output=True, text=True, timeout=5, shell=True)
    print(f"Exit code: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    print(f"Stderr: {result.stderr}")
    
    # Test multiple pipelines
    print("\n=== Testing multiple pipelines ===")
    tests = [
        ("echo test | echo test2", "Simple echo pipeline"),
        ("echo a && echo b | echo c", "Pipeline with &&"),
        ("echo x; echo y | echo z", "Pipeline with ;"),
    ]
    
    for cmd, desc in tests:
        print(f"\n{desc}: {cmd}")
        result = subprocess.run([rs_dash_path, "-c", cmd],
                               capture_output=True, text=True, timeout=2)
        print(f"  Exit: {result.returncode}, Output: {result.stdout.strip()}")

if __name__ == "__main__":
    test_user_scenario()