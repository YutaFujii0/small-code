# -----------------
# solution 1
# -----------------

# iterate through string
# when ([{ comes, push them
# when )]} comes, pop the opposite one,
# if it does not the corresponding brackets, immediately return false at that moment
# at the end, if stack has some brackets, return false, otherwise true
# time complexity: O(n)
# space complexity: possibly O(n)

class Solution:
    def isValid(self, s: str) -> bool:
        if len(s) % 2 == 1:
            return False
        stack = []
        popper = {
            ")": "(",
            "}": "{",
            "]": "["
        }
        for char in s:
            if char in popper.keys():
                if len(stack) == 0 or popper[char] != stack.pop():
                    return False
            else:
                stack.append(char)
        return len(stack) == 0

# solution2
# class Solution:
#     def isValid(self, s: str) -> bool:
#         l = len(s)
#         if l % 2 == 1:
#             return False
#         pop_map = {
#             ")": "(",
#             "]": "[",
#             "}": "{"
#         }
#         pop_keys = [")", "}", "]"]
#         i = 0
#         while True:
#             if s[i] in pop_keys:
#                 if i == 0 or s[i - 1] != pop_map[s[i]]:
#                     return False
#                 s = s[0:i-1] + s[i+1:]
#                 if s == "":
#                     return True
#                 i -= 1
#                 l -= 2
#             else:
#                 i += 1
#             if i >= l:
#                 return False
