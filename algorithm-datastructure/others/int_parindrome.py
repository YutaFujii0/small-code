# number could be negative, and it's an integer not a string
# it could be one digit

# naive solution
# - convert it to string
# - for each letter, pop first element and compare it with last
#   - if they are not the same, return false
#   - else continue
#   - pop those letters
# - return true
# time complexity O(n/2)
# space complexity O(n)

# -------------------------------
# solution 1
# -------------------------------

# class Solution:
#     def isPalindrome(self, x: int) -> bool:
#         str_x = list(str(x))
#         while len(str_x) > 1:
#             if str_x.pop(0) != str_x.pop():
#                 return False
#         return True

# -------------------------------
# solution 2
# -------------------------------

# copy original int and convert it to string then reverse convert it into int
# compare both and return if they are the same
# class Solution:
#     def isPalindrome(self, x: int) -> bool:
#         if x < 0:
#             return False
#         x_str = list(str(x))
#         x_str.reverse()
#         reversed_int = int("".join(x_str))
#         return x == reversed_int


# -------------------------------
# solution 3
# -------------------------------

import math

class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 10 and x >= 0:
            return True
        elif x < 0 or x % 10 == 0:
            return False

        reversed_x = 0
        while x > reversed_x and x >= 10:
            last_digit = x % 10
            x = math.floor(x / 10)
            reversed_x = reversed_x * 10 + last_digit

        if x == reversed_x:
            return True
        elif x == math.floor(reversed_x / 10):
            return True
        else:
            return False

