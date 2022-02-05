from typing import List
# property:
# idea: dynamic programming
# - tracking both min and max value


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        n = len(nums)
        minDp = [0] * n
        maxDp = [0] * n
        ans = nums[0]
        minDp[0] = maxDp[0] = nums[0]
        for i in range(1, n):
            x = minDp[i-1] * nums[i]
            y = maxDp[i-1] * nums[i]
            if nums[i] > 0:
                minDp[i] = min([x, nums[i]])
                maxDp[i] = max([y, nums[i]])
            else:
                minDp[i] = min([y, nums[i]])
                maxDp[i] = max([x, nums[i]])
            if maxDp[i] > ans:
                ans = maxDp[i]
        return ans
