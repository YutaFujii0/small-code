from typing import List
# Psuedocode
# i = len(nums) / 2
# stride = len(nums) / 2
# while stride > 0
# when nums[i] > target
# - i -= stride / 2
# when nums[i] < target
# - i += stride /2
# when nums[i] == target
# break and return i
# return i+1 if nums[i] < target else i

# n=4
# stride = 2 -> 1 -> 1 -> 1

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        stride = len(nums) // 2
        i = stride
        while stride > 0:
            stride //= 2
            if nums[i] > target:
                i -= stride
            elif nums[i] < target:
                i += stride
            elif nums[i] == target:
                return i
        return i+1 if nums[i] < target else i
