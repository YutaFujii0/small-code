# idea: stack
# psudedocode
# - positive = true
# - integer = 0
# - stack = []
# for char in s
#  - skip if char is a whitespace
#  - positive = false if '-' detected
#  if char is number
#    - push into stack
#  if char is not an integer
#    - break if stack is not empty
# for digit, i in stack
#   - (check if over)
#   - integer += digit * 10^i
#   
# return integer

class Solution:
    MAX_INT = pow(2,31) - 1
    MIN_INT = -pow(2,31)

    def myAtoi(self, s: str) -> int:
        sign = None
        integer = 0
        stack = []
        for c in s:
            if c.isdigit():
                stack.append(int(c))
                continue
            elif stack:
                break
            if c == " ":
                continue
            if c == "-" and sign == None:
                sign = -1
                stack.append(0)
                continue
            if c == "+" and sign == None:
                sign = 1
                stack.append(0)
                continue
            else:
                break
        
        i = 0
        if sign == -1:
            while stack:
                digit = stack.pop()
                diff = min([digit * 10**i, integer - self.MIN_INT])
                integer -= diff
                if integer == self.MIN_INT: break
                i += 1
        else:
            while stack:
                digit = stack.pop()
                diff = min([digit * 10**i, self.MAX_INT - integer])
                integer += diff
                if integer == self.MAX_INT: break
                i += 1
        return integer