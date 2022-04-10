# https://codeforces.com/contest/1665/problem/A
# example of content of input file
# 5
# 4
# 7
# 8
# 9
# 10
# Each test case contains a single line with integer 𝑛 (4≤𝑛≤10^9) — the sum of 𝑎, 𝑏, 𝑐, and 𝑑.

import sys

class Solution:
    @classmethod
    def call(cls, input):
        print(input)
        for text in [text.replace("\n", "") for text in input]:
            print(text)
            cls.solve(text)

    @classmethod
    def solve(cls, n: int) -> int:
        return 3


input = sys.stdin.readlines()
Solution.call(input)
