from typing import List

# if sign is positive
# - non zero minimum int, followed by all zeros, then put digits in increasing order
# if negative
# - put digits in descreasing order, followed by all zeros
# zeros array and digits array
# push digits into heap and zeros
# ans = 0
# if int is positive
#   while heap
#   - ans = ans * 10 + heappop()
#   n = zeros
#   - ans *= pow(10, n)
# else
#   while heap
#   - ans = ans * 10 + heappop()
#   n = zeros
#   - ans *= pow(10, n)

# time complexity: O(logn) * n for heap push + O(logn) * n for heap pop

import heapq

class Solution:
    def call(self, num: int) -> int:
        ans = 0
        zeros = 0
        digits = []
        if num > 0:
            while num != 0:
                val = num % 10
                num //= 10
                if val == 0:
                    zeros += 1
                else:
                    digits.append(val)
            heapq.heapify(digits)
            ans = heapq.heappop(digits)
            ans *= pow(10, zeros)
            while digits:
                ans = ans * 10 + heapq.heappop(digits)
        elif num < 0:
            num *= -1
            while num != 0:
                val = num % 10
                num //= 10
                if val == 0:
                    zeros += 1
                else:
                    digits.append(val * -1)
            heapq.heapify(digits)
            while digits:
                ans = ans * 10 + heapq.heappop(digits) * -1
            ans *= pow(10, zeros)
            ans *= -1
        else:
            return 0
        return ans


print(Solution().call(310))
print(Solution().call(-7605))
print(Solution().call(0))
print(Solution().call(0))
# print(Solution().call())
