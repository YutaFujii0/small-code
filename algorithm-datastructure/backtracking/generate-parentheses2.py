from typing import List
# idea: backtracking
# item: ["(", ")"]
# stop condition(success): length reaches 6 without being invalid
# stop condition(failure):
#  - more than 3 use of "(" or same goes for ")"
#  - parenthese is invalid

class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        items = ["(", ")"]
        validParentheses = []
        def backtrace(string):
            if string.count("(") > n or string.count(")") > n:
                return
            elif len(string) == n * 2:
                if valid(string):
                    validParentheses.append(string)
                return
            for char in items:
                string += char
                backtrace(string)
                string = string[:-1]
        def valid(parenthese: str) -> bool:
            stack = []
            for i in range(len(parenthese)):
                if parenthese[i] == "(":
                    stack.append(0)
                elif stack:
                    stack.pop()
                else:
                    return False
            return stack == []
        backtrace("")
        return validParentheses


# idea: backtracking
# item: ["(", ")"]
# stop condition(success): length reaches 6 without being invalid
# stop condition(failure):
#  - more than 3 use of "(" or same goes for ")"
#  - parenthese is invalid

class Solution2:
    def generateParenthesis(self, n: int) -> List[str]:
        validParentheses = []
        def backtrace(string: str, stackCount: int):
            if string.count("(") > n or string.count(")") > n:
                return
            elif len(string) == n * 2:
                validParentheses.append(string)
                return
            string += "("
            backtrace(string, stackCount + 1)
            string = string[:-1]
            if stackCount > 0:
                string += ")"
                backtrace(string, stackCount - 1)
                string = string[:-1]
        backtrace("", 0)
        return validParentheses
