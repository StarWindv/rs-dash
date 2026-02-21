#!/usr/bin/env python3
"""
Arithmetic Expansion Tests for rs-dash

Tests arithmetic expansion: $((expression))
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

def test_basic_arithmetic():
    """Test basic arithmetic operations"""
    print("=== Basic Arithmetic Operations ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Basic operations
        ("Addition", "echo $((1 + 2))", "3", 0),
        ("Subtraction", "echo $((5 - 3))", "2", 0),
        ("Multiplication", "echo $((2 * 3))", "6", 0),
        ("Division", "echo $((6 / 2))", "3", 0),
        ("Modulo", "echo $((7 % 3))", "1", 0),
        
        # Negative numbers
        ("Negative literal", "echo $((-5))", "-5", 0),
        ("Negative result", "echo $((2 - 5))", "-3", 0),
        
        # Zero
        ("Zero", "echo $((0))", "0", 0),
        
        # Large numbers
        ("Large number", "echo $((1000000))", "1000000", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_operator_precedence():
    """Test operator precedence"""
    print("\n=== Operator Precedence ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Multiplication before addition
        ("Precedence 1", "echo $((1 + 2 * 3))", "7", 0),  # 1 + (2*3) = 7
        ("Precedence 2", "echo $((2 * 3 + 4))", "10", 0), # (2*3) + 4 = 10
        
        # Parentheses
        ("Parentheses 1", "echo $(( (1 + 2) * 3 ))", "9", 0),  # (1+2)*3 = 9
        ("Parentheses 2", "echo $(( 2 * (3 + 4) ))", "14", 0), # 2*(3+4) = 14
        
        # Nested parentheses
        ("Nested parentheses", "echo $(( ( (1 + 2) * 3 ) + 4 ))", "13", 0), # ((1+2)*3)+4 = 13
        
        # Modulo and division
        ("Mixed precedence", "echo $(( 2 + 8 / 2 * 3 ))", "14", 0), # 2 + (8/2)*3 = 14
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_bitwise_operations():
    """Test bitwise operations"""
    print("\n=== Bitwise Operations ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Bitwise AND
        ("Bitwise AND", "echo $((5 & 3))", "1", 0),  # 0101 & 0011 = 0001
        
        # Bitwise OR
        ("Bitwise OR", "echo $((5 | 2))", "7", 0),   # 0101 | 0010 = 0111
        
        # Bitwise XOR
        ("Bitwise XOR", "echo $((5 ^ 3))", "6", 0),  # 0101 ^ 0011 = 0110
        
        # Bitwise NOT
        ("Bitwise NOT", "echo $((~0))", "-1", 0),    # ~0 = -1 (two's complement)
        ("Bitwise NOT 2", "echo $((~1))", "-2", 0),  # ~1 = -2
        
        # Left shift
        ("Left shift", "echo $((1 << 3))", "8", 0),  # 1 << 3 = 8
        ("Left shift 2", "echo $((3 << 2))", "12", 0), # 3 << 2 = 12
        
        # Right shift
        ("Right shift", "echo $((8 >> 2))", "2", 0),  # 8 >> 2 = 2
        ("Right shift 2", "echo $((15 >> 1))", "7", 0), # 15 >> 1 = 7
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_logical_operations():
    """Test logical operations"""
    print("\n=== Logical Operations ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Logical AND (&&)
        ("Logical AND true", "echo $((1 && 1))", "1", 0),
        ("Logical AND false", "echo $((1 && 0))", "0", 0),
        ("Logical AND both false", "echo $((0 && 0))", "0", 0),
        
        # Logical OR (||)
        ("Logical OR true", "echo $((1 || 0))", "1", 0),
        ("Logical OR false", "echo $((0 || 0))", "0", 0),
        ("Logical OR both true", "echo $((1 || 1))", "1", 0),
        
        # Logical NOT (!)
        ("Logical NOT true", "echo $((!0))", "1", 0),
        ("Logical NOT false", "echo $((!1))", "0", 0),
        ("Logical NOT non-zero", "echo $((!5))", "0", 0),  # Any non-zero is true, !true = false
        
        # Comparison operators (return 1 for true, 0 for false)
        ("Equal", "echo $((2 == 2))", "1", 0),
        ("Not equal", "echo $((2 != 3))", "1", 0),
        ("Less than", "echo $((2 < 3))", "1", 0),
        ("Less or equal", "echo $((2 <= 2))", "1", 0),
        ("Greater than", "echo $((3 > 2))", "1", 0),
        ("Greater or equal", "echo $((3 >= 3))", "1", 0),
        
        # False comparisons
        ("Equal false", "echo $((2 == 3))", "0", 0),
        ("Greater false", "echo $((2 > 3))", "0", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_ternary_operator():
    """Test ternary conditional operator"""
    print("\n=== Ternary Operator ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Ternary operator ?:
        ("Ternary true", "echo $((1 ? 100 : 200))", "100", 0),
        ("Ternary false", "echo $((0 ? 100 : 200))", "200", 0),
        
        # Nested ternary
        ("Nested ternary", "echo $((1 ? 2 ? 3 : 4 : 5))", "3", 0),  # 1 ? (2 ? 3 : 4) : 5
        
        # Complex conditions
        ("Complex ternary", "echo $(( (2 > 1) ? 10 : 20 ))", "10", 0),
        ("Complex ternary 2", "echo $(( (2 < 1) ? 10 : 20 ))", "20", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_variables_in_arithmetic():
    """Test variables in arithmetic expressions"""
    print("\n=== Variables in Arithmetic ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Simple variable
        ("Variable addition", "A=5; echo $((A + 3))", "8", 0),
        ("Variable multiplication", "B=3; echo $((B * 4))", "12", 0),
        
        # Multiple variables
        ("Multiple variables", "X=2; Y=3; echo $((X * Y))", "6", 0),
        
        # Variable assignment in arithmetic
        ("Arithmetic assignment", "C=5; echo $((C += 3))", "8", 0),
        ("Check assignment", "C=5; C=$((C + 3)); echo $C", "8", 0),
        
        # Increment/decrement
        ("Post-increment", "D=5; echo $((D++))", "5", 0),  # Returns 5, then D becomes 6
        ("Pre-increment", "E=5; echo $((++E))", "6", 0),   # E becomes 6, returns 6
        
        # Undefined variable (should be 0)
        ("Undefined variable", "echo $((UNDEF + 5))", "5", 0),
        ("Undefined in expr", "echo $((UNDEF))", "0", 0),
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_number_bases():
    """Test different number bases"""
    print("\n=== Number Bases ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Decimal (default)
        ("Decimal", "echo $((10))", "10", 0),
        
        # Hexadecimal
        ("Hex lowercase", "echo $((0x10))", "16", 0),   # 0x10 = 16
        ("Hex uppercase", "echo $((0XFF))", "255", 0),  # 0xFF = 255
        ("Hex with letters", "echo $((0x1A))", "26", 0), # 0x1A = 26
        
        # Octal
        ("Octal", "echo $((010))", "8", 0),    # 010 = 8
        ("Octal large", "echo $((0777))", "511", 0), # 0777 = 511
        
        # Mixed bases in expressions
        ("Mixed bases", "echo $((0x10 + 010))", "24", 0),  # 16 + 8 = 24
        
        # Base conversion
        ("Base conversion", "echo $((16#FF))", "255", 0),  # Base 16: FF = 255
        ("Base 2", "echo $((2#1010))", "10", 0),          # Binary: 1010 = 10
        ("Base 8", "echo $((8#77))", "63", 0),            # Octal: 77 = 63
    ]
    
    passed = 0
    failed = 0
    
    for name, cmd, expected, exit_code in tests:
        if runner.run_test(name, cmd, exit_code, expected):
            passed += 1
        else:
            failed += 1
    
    return passed, failed

def test_edge_cases():
    """Test edge cases and error conditions"""
    print("\n=== Edge Cases ===")
    
    runner = TestRunner(verbose=True)
    
    tests = [
        # Empty expression
        ("Empty expression", "echo $(())", "0", 0),  # Empty arithmetic expands to 0
        
        # Division by zero (should error)
        ("Division by zero", "echo $((1 / 0))", "", 1),
        ("Modulo by zero", "echo $((1 % 0))", "", 1),
        
        # Large expressions
        ("Large expression", "echo $((1+2+3+4+5+6+7+8+9+10))", "55", 0),
        
        # Whitespace in expression
        ("Whitespace", "echo $(( 1 + 2 ))", "3", 0),
        ("Extra whitespace", "echo $((  1   +   2   ))", "3", 0),
        
        # Nested arithmetic
        ("Nested arithmetic", "echo $(( $((1 + 2)) * 3 ))", "9", 0),
        
        # Command substitution in arithmetic
        ("Command sub in arithmetic", "echo $(( $(echo 5) + 3 ))", "8", 0),
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
    """Run all arithmetic expansion tests"""
    print("Arithmetic Expansion Tests for rs-dash")
    print("=" * 60)
    print("Testing arithmetic expansion: $((expression))")
    print("=" * 60)
    
    total_passed = 0
    total_failed = 0
    
    # Run test suites
    test_suites = [
        ("Basic Arithmetic", test_basic_arithmetic),
        ("Operator Precedence", test_operator_precedence),
        ("Bitwise Operations", test_bitwise_operations),
        ("Logical Operations", test_logical_operations),
        ("Ternary Operator", test_ternary_operator),
        ("Variables in Arithmetic", test_variables_in_arithmetic),
        ("Number Bases", test_number_bases),
        ("Edge Cases", test_edge_cases),
    ]
    
    for name, test_func in test_suites:
        print(f"\nRunning {name} tests...")
        passed, failed = test_func()
        total_passed += passed
        total_failed += failed
        print(f"{name}: {passed} passed, {failed} failed")
    
    # Summary
    print("\n" + "=" * 60)
    print("ARITHMETIC EXPANSION TEST SUMMARY")
    print("=" * 60)
    print(f"Total tests: {total_passed + total_failed}")
    print(f"Passed: {total_passed}")
    print(f"Failed: {total_failed}")
    
    if total_failed > 0:
        print("\nSome arithmetic expansion tests failed!")
        sys.exit(1)
    else:
        print("\nAll arithmetic expansion tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main()