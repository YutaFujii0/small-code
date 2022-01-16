from typing import List
# idea: backtracking
# item: [(,(,(,),),)]
# successful break condition: none
# failure condition: nice to have
# end: item exhausted
# after finish, evaluate?

# evaluation: using stack
# for p in combinations
#   - push if p == "("
#   - pop if p == ")", return false on error
# return true if stack is empty

# if this stack is incorporated into backtracing, we can detect failure within the process
# Complexity:
# time complexity & space complexity
#  - candidates generation: O(n!)
#  - candidates validation: O(n!)


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        items = ["(" for _ in range(n)] + [")" for _ in range(n)]
        def backtrace(items: List[str]) -> List[str]:
            n = len(items)
            if n == 0:
                return [""]
            results = []
            for i in range(n):
                for phrase in backtrace(items[:i]+items[i+1:]):
                    results.append(phrase + items[i])
            return results
        candidates = backtrace(items)

        def valid(string):
            stack = []
            for i in range(len(string)):
                if string[i] == "(":
                    stack.append(0)
                elif stack:
                    stack.pop()
                else:
                    return False
            return stack == []
        validSet = set()
        for candidate in candidates:
            if valid(candidate):
                validSet.add(candidate)
        return validSet
