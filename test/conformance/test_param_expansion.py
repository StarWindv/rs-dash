#!/usr/bin/env python3
"""
Parameter Expansion Tests for rs-dash

Tests various forms of parameter expansion:
- ${VAR} basic expansion
- ${VAR:-default} use default value
- ${VAR:=default} assign default value
- ${VAR:?error} error if unset
- ${VAR:+alternate} use alternate if set
- ${#VAR} string length
- ${VAR%pattern} remove suffix
- ${VAR#pattern} remove prefix
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

def test_basic_param_expansion():
    """Test ${VAR} basic expansion"""
    print("=== Basic Parameter Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Basic ${VAR} expansion
        ("Basic ${}", "VAR=test; echo ${VAR}", "test", 0),
        ("Braces optional", "VAR=test; echo $VAR", "test", 0),
        
        # Variable with special characters in value
        ("Special chars", 'VAR="hello world"; echo "${VAR}"', "hello world", 0),
        ("Multiple words", 'VAR="multiple words"; echo ${VAR}', "multiple words", 0),
        
        # Undefined variable
        ("Undefined var", "echo ${UNDEF}", "", 0),
        ("Undefined with braces", "echo ${UNDEFINED_VAR}", "", 0),
        
        # Variable following immediately
        ("Immediate following", "VAR=test; echo ${VAR}ing", "testing", 0),
        
        # Nested braces (invalid, should error or treat as literal)
        ("Nested braces", "echo ${VAR}}", "}", 0),  # Should output just }
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_default_value_expansion():
    """Test ${VAR:-word} and ${VAR:=word}"""
    print("\n=== Default Value Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR:-default} - use default if unset or null
        ("Use default unset", "echo ${UNDEF:-default}", "default", 0),
        ("Use default empty", "EMPTY=; echo ${EMPTY:-default}", "default", 0),
        ("Don't use default", "VAR=value; echo ${VAR:-default}", "value", 0),
        
        # ${VAR:=default} - assign default if unset or null
        ("Assign default unset", "echo ${UNDEF:=assigned}; echo $UNDEF", "assigned\nassigned", 0),
        ("Assign default empty", "EMPTY=; echo ${EMPTY:=assigned}; echo $EMPTY", "assigned\nassigned", 0),
        ("Don't assign default", "VAR=value; echo ${VAR:=default}; echo $VAR", "value\nvalue", 0),
        
        # Default with spaces
        ("Default with spaces", 'echo ${UNDEF:-default value}', "default value", 0),
        ("Default with special chars", 'echo ${UNDEF:-a=b}', "a=b", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_error_if_unset_expansion():
    """Test ${VAR:?error}"""
    print("\n=== Error If Unset Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR:?message} - error if unset or null
        ("Error unset", "echo ${UNDEF:?error}", "", 1),
        ("Error empty", "EMPTY=; echo ${EMPTY:?error}", "", 1),
        ("No error set", "VAR=value; echo ${VAR:?error}", "value", 0),
        
        # Custom error message
        ("Custom message", "echo ${UNDEF:?Variable is unset}", "", 1),
        ("Empty message", "echo ${UNDEF:?}", "", 1),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_alternate_value_expansion():
    """Test ${VAR:+word}"""
    print("\n=== Alternate Value Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR:+word} - use alternate if set and non-null
        ("Alternate unset", "echo ${UNDEF:+alternate}", "", 0),
        ("Alternate empty", "EMPTY=; echo ${EMPTY:+alternate}", "", 0),
        ("Alternate set", "VAR=value; echo ${VAR:+alternate}", "alternate", 0),
        
        # Alternate with variable reference
        ("Alternate with var", "VAR=value; ALT=other; echo ${VAR:+$ALT}", "other", 0),
        ("Alternate complex", 'VAR=set; echo ${VAR:+is ${VAR}}', "is set", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_string_length_expansion():
    """Test ${#VAR} string length"""
    print("\n=== String Length Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${#VAR} - string length
        ("Length empty", "EMPTY=; echo ${#EMPTY}", "0", 0),
        ("Length short", "VAR=a; echo ${#VAR}", "1", 0),
        ("Length normal", "VAR=test; echo ${#VAR}", "4", 0),
        ("Length with spaces", 'VAR="hello world"; echo ${#VAR}', "11", 0),
        ("Length unset", "echo ${#UNDEF}", "0", 0),
        
        # Length of positional parameters
        ("Length of $@", 'echo ${#@}', "0", 0),  # Should be 0 with no args
        ("Length of $*", 'echo ${#*}', "0", 0),  # Should be 0 with no args
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_pattern_removal_expansion():
    """Test ${VAR%pattern} and ${VAR#pattern}"""
    print("\n=== Pattern Removal Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR%pattern} - remove smallest suffix pattern
        ("Remove suffix", "VAR=filename.txt; echo ${VAR%.txt}", "filename", 0),
        ("Remove suffix no match", "VAR=filename; echo ${VAR%.txt}", "filename", 0),
        ("Remove shortest suffix", "VAR=file.tar.gz; echo ${VAR%.*}", "file.tar", 0),
        
        # ${VAR%%pattern} - remove largest suffix pattern
        ("Remove largest suffix", "VAR=file.tar.gz; echo ${VAR%%.*}", "file", 0),
        
        # ${VAR#pattern} - remove smallest prefix pattern
        ("Remove prefix", "VAR=filename.txt; echo ${VAR#file}", "name.txt", 0),
        ("Remove prefix no match", "VAR=filename; echo ${VAR#test}", "filename", 0),
        ("Remove shortest prefix", "VAR=/path/to/file; echo ${VAR#/}", "path/to/file", 0),
        
        # ${VAR##pattern} - remove largest prefix pattern
        ("Remove largest prefix", "VAR=/path/to/file; echo ${VAR##*/}", "file", 0),
        
        # Special patterns
        ("Remove all suffix", "VAR=test.test.test; echo ${VAR%%.test}", "test", 0),
        ("Remove all prefix", "VAR=test.test.test; echo ${VAR##test.}", "test.test", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_pattern_substitution_expansion():
    """Test ${VAR/pattern/replacement}"""
    print("\n=== Pattern Substitution Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR/pattern/replacement} - replace first occurrence
        ("Replace first", "VAR=hello world; echo ${VAR/world/everyone}", "hello everyone", 0),
        ("Replace first no match", "VAR=hello; echo ${VAR/world/everyone}", "hello", 0),
        ("Replace empty", "VAR=test; echo ${VAR//e}", "tst", 0),  # Remove 'e'
        
        # ${VAR//pattern/replacement} - replace all occurrences
        ("Replace all", "VAR=test test; echo ${VAR//test/example}", "example example", 0),
        ("Replace all multiple", "VAR=a.b.c.d; echo ${VAR//./-}", "a-b-c-d", 0),
        
        # Anchored patterns
        ("Replace start", "VAR=testtest; echo ${VAR/#test/example}", "exampletest", 0),
        ("Replace end", "VAR=testtest; echo ${VAR/%test/example}", "testexample", 0),
        
        # Special characters in pattern
        ("Replace dot", "VAR=file.txt; echo ${VAR/.txt/.bak}", "file.bak", 0),
        ("Replace star", "VAR=a*b*c; echo ${VAR/*b*/replaced}", "replacedc", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_case_modification_expansion():
    """Test ${VAR^}, ${VAR^^}, ${VAR,}, ${VAR,,}"""
    print("\n=== Case Modification Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${VAR^} - uppercase first character
        ("Uppercase first", "VAR=test; echo ${VAR^}", "Test", 0),
        ("Uppercase first already", "VAR=Test; echo ${VAR^}", "Test", 0),
        
        # ${VAR^^} - uppercase all characters
        ("Uppercase all", "VAR=test; echo ${VAR^^}", "TEST", 0),
        ("Uppercase all mixed", "VAR=TeSt; echo ${VAR^^}", "TEST", 0),
        
        # ${VAR,} - lowercase first character
        ("Lowercase first", "VAR=TEST; echo ${VAR,}", "tEST", 0),
        ("Lowercase first already", "VAR=test; echo ${VAR,}", "test", 0),
        
        # ${VAR,,} - lowercase all characters
        ("Lowercase all", "VAR=TEST; echo ${VAR,,}", "test", 0),
        ("Lowercase all mixed", "VAR=TeSt; echo ${VAR,,}", "test", 0),
        
        # With empty string
        ("Case empty", "EMPTY=; echo ${EMPTY^}", "", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_indirect_expansion():
    """Test ${!VAR} indirect expansion"""
    print("\n=== Indirect Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # ${!VAR} - indirect expansion (value of variable named by VAR)
        ("Indirect simple", "NAME=VAR; VAR=value; echo ${!NAME}", "value", 0),
        ("Indirect chain", "A=B; B=C; C=value; echo ${!A}", "C", 0),  # ${!A} = ${B} = C
        ("Indirect unset", "NAME=UNDEF; echo ${!NAME}", "", 0),
        ("Indirect empty", "NAME=EMPTY; EMPTY=; echo ${!NAME}", "", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_array_expansion():
    """Test array variable expansion"""
    print("\n=== Array Expansion ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Note: Arrays are not POSIX but common in bash/dash
        # Basic array (if supported)
        ("Array element", 'ARR[0]=first; ARR[1]=second; echo ${ARR[0]}', "first", 0),
        ("All array elements", 'ARR[0]=a; ARR[1]=b; echo ${ARR[@]}', "a b", 0),
        ("Array length", 'ARR[0]=a; ARR[1]=b; echo ${#ARR[@]}', "2", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def main():
    """Run all parameter expansion tests"""
    print("Parameter Expansion Tests for rs-dash")
    print("=" * 60)
    print("Testing various forms of parameter expansion")
    print("=" * 60)
    
    total_passed = 0
    total_failed = 0
    
    # Run test suites
    test_suites = [
        ("Basic Expansion", test_basic_param_expansion),
        ("Default Values", test_default_value_expansion),
        ("Error If Unset", test_error_if_unset_expansion),
        ("Alternate Values", test_alternate_value_expansion),
        ("String Length", test_string_length_expansion),
        ("Pattern Removal", test_pattern_removal_expansion),
        ("Pattern Substitution", test_pattern_substitution_expansion),
        ("Case Modification", test_case_modification_expansion),
        ("Indirect Expansion", test_indirect_expansion),
        ("Array Expansion", test_array_expansion),
    ]
    
    for name, test_func in test_suites:
        print(f"\nRunning {name} tests...")
        passed, failed = test_func()
        total_passed += passed
        total_failed += failed
        print(f"{name}: {passed} passed, {failed} failed")
    
    # Summary
    print("\n" + "=" * 60)
    print("PARAMETER EXPANSION TEST SUMMARY")
    print("=" * 60)
    print(f"Total tests: {total_passed + total_failed}")
    print(f"Passed: {total_passed}")
    print(f"Failed: {total_failed}")
    
    if total_failed > 0:
        print("\nSome parameter expansion tests failed!")
        sys.exit(1)
    else:
        print("\nAll parameter expansion tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main()