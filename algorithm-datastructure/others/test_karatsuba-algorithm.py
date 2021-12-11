# karatsuba's product algorithm
# - aplit x to a * 10^n/2 + b
# - aplit y to c * 10^n/2 + d
# - 1.a*c
# - 2.b*d
# - 3.(a+b)(c+d) - a*c - b*d
# 1 * 10^n + 3 * 10^n/2 + 2
# Complexity
# - time complexity: sum k of 3 * O((n/2^k)^2)
# - space complexity: O(logn)

import pytest
import math


def product(x: int, y: int) -> int:
    # print(f'{x} * {y}')
    digits = 1 if x == 0 else int(math.log10(x))+1
    if digits <= 1:
        return x * y
    split_digit = 10**(digits//2)
    b = x % split_digit
    a = (x - b) // split_digit
    d = y % split_digit
    c = (y - d) // split_digit
    ac = product(a, c)
    bd = product(b, d)
    abcd = product(a+b, c+d) - ac - bd
    return ac * 10**(digits//2 * 2) + abcd * 10**(digits//2) + bd

a = 3141592653589793238462643383279502884197169399375105820974944592
b = 2718281828459045235360287471352662497757247093699959574966967627
print(product(a,b))

def test_1():
    assert product(1, 1) == 1

def test_2():
    assert product(2, 3) == 6

def test_3():
    assert product(2, 8) == 16

def test_4():
    assert product(12, 8) == 96

def test_5():
    assert product(12, 12) == 144

def test_6():
    assert product(365, 24) == 8760

def test_7():
    assert product(1234, 5678) == 7006652

def test_8():
    assert product(36, 2) == 72

def test_9():
    assert product(41, 6) == 246
