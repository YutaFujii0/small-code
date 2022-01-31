# property: reverse=first in last out
# idea: stack
# Complexity:
# - time complexity: O(1) * n
# - space complexity: O(n) for stack

class Solution:
    def reverse(self, x: int) -> int:        
        MAX_INT = pow(2, 31) - 1
        MIN_INT = -pow(2, 31)

        sign = 1
        str_x = str(x)
        if str_x[0] == '-':
            sign = -1
            str_x = str_x[1:]
        x_reversed = 0
        if sign == 1:
            for i, digit in enumerate(str_x):
                diff = int(digit) * pow(10, i)
                if diff > MAX_INT - x_reversed: return 0
                x_reversed += diff
        else:
            for i, digit in enumerate(str_x):
                diff = int(digit) * pow(10, i)
                if diff > x_reversed - MIN_INT: return 0
                x_reversed -= diff
        return x_reversed
