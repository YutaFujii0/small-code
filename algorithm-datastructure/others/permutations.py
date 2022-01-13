from typing import List
# permutation
# n = 3
# 1,2,3 / 1,3,2 / 
# mutate and push? finish condition?
# idea: recursion
# result = []
# for i in nums
#  - for arr in permutate(nums - i)
#    - arr.push(i)
#    - result.push(arr)
# Complexity:
# - time complexity: n * n-1 * ...: O(n!)
# - space complexity: O(n!) for each array

class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 1:
            return [nums]
        result = []
        for i in range(len(nums)):
            for item in self.permute(nums[:i] + nums[i+1:]):
                item.append(nums[i])
                result.append(item)
        return result
