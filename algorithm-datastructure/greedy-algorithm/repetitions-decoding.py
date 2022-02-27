import sys

class Solution:
    def call(self, input):
        problems = self.parse(input)
        print(problems)
        for problem in problems:
            self.solve(problem)
    
    def parse(self, input):
        raw = [text.split(" ") for text in input]
        # omit first line
        raw.pop(0)
        problems = []
        for i in range(len(raw)//2):
            problems.append({ "n": int(raw[2*i][0]), "nums": [int(a) for a in raw[2*i+1]] })
        return problems
    
    # impossible if frequency of any num is odd
    # impossible if I need to reproduce such tandem from scrach that is not palindrome
    # pattern) xyyx -> xyxxyx
    # if I need to duplicate tandem xyzxyz
    def solve(self, problem):

        return 0

# input format
# 4
# 2
# 5 7
# 2
# 5 5
# 6
# 1 3 1 2 2 3
# 6
# 3 2 1 1 2 3

input = sys.stdin.readlines()
Solution().call(input)