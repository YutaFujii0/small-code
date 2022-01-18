from typing import List
# naive idea: remove zero and push back
# cons: remove from array usually takes O(n) at worst
# swap would be cost effective


class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        i = 0
        n = len(nums)
        while n > 0:
            if nums[i] == 0:
                nums.append(nums.pop(i))
            else:
                i += 1
            n -= 1
