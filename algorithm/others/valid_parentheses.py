# 1115 1135
# maybe no parentheses contained
# it's kind of like a parser or interpreter
# using stack feels good

# -----------------
# solution 1
# -----------------

# iterate through string
# when ([{ comes, push them
# when )]} comes, pop the opposite one,
# if it does not the corresponding brackets, immediately return false at that moment
# at the end, if stack has some brackets, return false, otherwise true

# test case
# ")" => true
# "()" => true
# "[]" => true
# "{}" => true
# "(" => false
# ")" => false
# "]" => false
# "((((()))))" => true
# "((([()])))" => true
# "((((()])))" => false

# time complexity: O(n)
# space complexity: possibly O(n)

# class Solution:
#     def isValid(self, s: str) -> bool:
#         if len(s) % 2 == 1:
#             return False
#         stack = []
#         pop_map = {
#             ")": "(",
#             "]": "[",
#             "}": "{"
#         }
#         for i in range(len(s)):
#             if s[i] in ["(", "[", "{"]:
#                 stack.append(s[i])
#             elif s[i] in pop_map:
#                 if len(stack) == 0 or stack.pop() != pop_map[s[i]]:
#                     return False
#         return len(stack) == 0

    
# solution2
class Solution:
    def isValid(self, s: str) -> bool:
        l = len(s)
        if l % 2 == 1:
            return False
        pop_map = {
            ")": "(",
            "]": "[",
            "}": "{"
        }
        pop_keys = [")", "}", "]"]
        i = 0
        while True:
            if s[i] in pop_keys:
                if i == 0 or s[i - 1] != pop_map[s[i]]:
                    return False
                s = s[0:i-1] + s[i+1:]
                if s == "":
                    return True
                i -= 1
                l -= 2
            else:
                i += 1
            if i >= l:
                return False
