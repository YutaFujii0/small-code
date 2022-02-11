from typing import List
# property: distance, min
# naive solution: O(n^2) comparison

# divide and conquer?
# where to divide?

# dp?
# base case is not easy

# two pointer
# idea: starting from edge telements,
# by shrinking pointers' length, if i move pointer of shorter length,
# that would not make greater area
# Time complexity: O(n)
# Space complexity: O(1)


class Solution:
    def maxArea(self, height: List[int]) -> int:
        left = 0
        right = len(height) - 1
        localMax = min(height[left], height[right]) * (right - left)
        while (right - left) > 1:
            if height[left] > height[right]:
                right -= 1
            else:
                left += 1
            area = min(height[left], height[right]) * (right - left)
            localMax = max(area, localMax)
        return localMax
