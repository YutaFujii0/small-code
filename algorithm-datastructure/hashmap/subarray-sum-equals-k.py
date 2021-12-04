from typing import List

# https://leetcode.com/problems/subarray-sum-equals-k/submissions/

# O(n) Solution
# Basic idea:
# define S_i := sum of nums[0:i+1]
# then, the problem becomes how many i,j pairs s.t S_j - S_i = k
# it just becomes two sum problem of array [S_0, S_1, ..., S_n]

# Psuedocode
# meets = 0
# for i in length of nums
#  - calculate cumulative sum to ith element
#  - countup that element of hash
#  - if not
#   - insert
#   - lookup count of hash[k - sum]
#   - add that to meets
# Time complexity: n * O(1)
# Space complexity: O(n) for extra hashmap

class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        meets = 0
        cum_sums = {0: 1}
        current_sum = 0
        for i in range(len(nums)):
            current_sum += nums[i]
            if current_sum - k in cum_sums.keys():
                meets += cum_sums[current_sum - k]
            if current_sum in cum_sums.keys():
                cum_sums[current_sum] += 1
            else:
                cum_sums[current_sum] = 1
        return meets
