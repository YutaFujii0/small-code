from typing import List
# idea: sliding window
# left, right = 0, 0
# minLen = infty
# localSum = 0
# while right < end of nums
#  - localSum += nums[right]
#  - if localSum >= targed (window slides!)
#    - while localSum >= target
#      - localSum -= nums[left]
#      - left++
#    - minLen = min([minLen, right - left + 1])
#  - right++
# final comparison
# return
# Complexity
# - time complexity: n times sliding of right/left, and comparisons: O(3n)
# - space complexity: O(1)

class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        length = len(nums)
        left = 0
        right = 0
        minLen = length + 1
        localSum = 0
        while right < length:
            localSum += nums[right]
            if localSum >= target:
                while localSum >= target and left <= right:
                    localSum -= nums[left]
                    left += 1
                minLen = min([minLen, right - left + 2])
            right += 1
        if minLen > length:
            return 0
        return minLen
