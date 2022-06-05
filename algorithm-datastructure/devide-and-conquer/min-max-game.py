from typing import List

class Solution:
    def minMaxGame(self, nums: List[int]) -> int:
        n = len(nums)
        while n > 1:
            newNums = []
            for i in range(n // 2):
                el = min(nums[2 * i], nums[2 * i + 1]) if i % 2 == 0 else max(nums[2 * i], nums[2 * i + 1])
                newNums.append(el)
            nums = newNums
            n //= 2
        return nums[0]