from typing import List
# action: multiple lookups
# idea: hash
# Psuedocode
# - insert nums into hash
# while hash contains original
#  - original *= 2
# - return original

class Solution:
    def findFinalValue(self, nums: List[int], original: int) -> int:
        nums = set(nums)
        while original in nums:
            original *= 2
        return original
