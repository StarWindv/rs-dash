#!/usr/bin/env python3
"""
Run all tests for rs-dash
Main test runner script
"""

import sys
import os
import subprocess

def run_test_suite(name, test_path):
    """Run a test suite and return results"""
    print(f"\n{'='*60}")
    print(f"Running {name} tests")
    print(f"{'='*60}")
    
    try:
        result = subprocess.run([sys.executable, test_path], 
                              capture_output=True, text=True)
        
        print(result.stdout)
        if result.stderr:
            print(f"Stderr: {result.stderr}")
        
        return result.returncode == 0
        
    except Exception as e:
        print(f"Error running {name}: {e}")
        return False

def main():
    """Run all test suites"""
    print("rs-dash Comprehensive Test Suite")
    print("=" * 60)
    
    # Check if rs-dash is built
    print("Checking if rs-dash is built...")
    
    # First, try to build if not exists
    target_dir = os.path.join(os.path.dirname(__file__), "..", "target", "debug")
    rs_dash_exe = "rs-dash.exe" if os.name == 'nt' else "rs-dash"
    rs_dash_path = os.path.join(target_dir, rs_dash_exe)
    
    if not os.path.exists(rs_dash_path):
        print("rs-dash not found. Building...")
        try:
            result = subprocess.run(["cargo", "build"], 
                                  capture_output=True, text=True)
            if result.returncode != 0:
                print(f"Build failed: {result.stderr}")
                return 1
            print("Build successful!")
        except Exception as e:
            print(f"Error building rs-dash: {e}")
            return 1
    
    # Get absolute paths to test files
    test_dir = os.path.dirname(__file__)
    
    test_suites = [
        ("Unit", os.path.join(test_dir, "unit", "test_basic.py")),
        ("Integration", os.path.join(test_dir, "integration", "test_integration.py")),
        ("Regression", os.path.join(test_dir, "regression", "test_regression.py")),
    ]
    
    results = {}
    
    # Run each test suite
    for name, path in test_suites:
        if os.path.exists(path):
            success = run_test_suite(name, path)
            results[name] = success
        else:
            print(f"Test suite not found: {path}")
            results[name] = False
    
    # Summary
    print("\n" + "=" * 60)
    print("TEST SUITE SUMMARY")
    print("=" * 60)
    
    all_passed = True
    for name, success in results.items():
        status = "PASS" if success else "FAIL"
        print(f"{name:15} {status}")
        if not success:
            all_passed = False
    
    print("\n" + "=" * 60)
    if all_passed:
        print("All test suites passed!")
        return 0
    else:
        print("Some test suites failed!")
        return 1

if __name__ == "__main__":
    sys.exit(main())