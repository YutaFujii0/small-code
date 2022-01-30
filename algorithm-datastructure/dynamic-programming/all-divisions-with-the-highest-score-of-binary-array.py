from typing import List
# property of problem: easily calculate kth score if k-1 is known
# idea: dynamic programming
# Psuedocode
# - dp = [0] * len(nums)
# - dp[0] = count 0
# - localMax = dp[0]
# - hash = {dp[0]: [0]}
# for i in 1..n
#   if nums[i] == 0
#     - dp[i] = dp[i-1] + 1
#   else
#     - dp[i] = dp[i-1] - 1
#   localMax = dp[i] if dp[i] is higher
#   push key dp[i] value i into hash
# return hash[localMax]
# Time complexity: n * [O(1) + O(1) + O(1)] for dp
# Space complexity: O(2n)

class Solution:
    def maxScoreIndices(self, nums: List[int]) -> List[int]:
        n = len(nums)
        dp = [0] * (n+1)
        dp[0] = sum(nums)
        localMax = dp[0]
        indexGroup = { dp[0]: [0] }
        for i in range(1, n+1):
            if nums[i-1] == 0:
                dp[i] = dp[i-1] + 1
            else:
                dp[i] = dp[i-1] - 1
            if dp[i] > localMax:
                localMax = dp[i]
            if dp[i] in indexGroup.keys():
                indexGroup[dp[i]].append(i)
            else:
                indexGroup[dp[i]] = [i]
        return indexGroup[localMax]
                