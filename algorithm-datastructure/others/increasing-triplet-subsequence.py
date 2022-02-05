from typing import List
# [5,6,4,7] -> true
# [5,6,1,2,3] -> true
# naive solution, nC3 combinations, O(n^3) time complexity

# property: order does matter
# 
# idea: keep minimum 3 elements
# [min1, min2]
# if i > min2, return true
# else if i > min1, min2= i
# else min1 = i
# n * O(3) time complexity
# O(1) space complexity

class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        min1, min2 = nums[0], pow(2,31)
        for i in nums[1:]:
            if i > min2:
                return True
            elif i > min1:
                min2 = i
            else:
                min1 = i
        return False
