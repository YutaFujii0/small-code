# https://leetcode.com/problems/longest-increasing-subsequence/

from typing import List
# if some node k has already predecessor, then no extra traversal needed
# - from this node
# Psuedocode
# explored = hash(index, max)
# for num in nums
# - traverse
# return max of explored hash values

# def traverse(index, nums)
# array of local max = []
# for index <= i < len(nums)
# - if i explored
#  - push local max to max candidates
# - else
#  - recurse nums[i]
# mark i as explored (with value max of local max + 1)
# return that value

# Time Complexity:
# revursive call:
# T(n) = T(n-1) * k and <local procedure> * (k -1)
# where k: # of values 
#  - after index n
#  - yet explored
#  - value is strictly larger than N-nth element
# <local procedure>:O(1)
#  - lookup from hash table - O(1)
#  - insert into hash table - O(1)
# retrieve max: O(n)

class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        self.explored = set()
        self.max = {}
        def traverse(index, nums):
            if index in self.explored:
                return self.max[index]
            candidates = [0]
            for i in range(index, len(nums)):
                if nums[index] < nums[i]:
                    if i in self.explored:
                        candidates.append(self.max[i])
                    else:
                        candidates.append(traverse(i, nums))
            self.explored.add(index)
            maxOfMax = max(candidates)
            self.max[index] = maxOfMax + 1
            return maxOfMax + 1

        for i in range(len(nums)):
            traverse(i, nums)
        return max(self.max.values())
