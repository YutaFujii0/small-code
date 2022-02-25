from typing import List

# basic idea: backtracking
# stop condition: num of elements become 2
# 
# def backtrack(arr, nums)
# return if len(arr) = 2
# for i in nums
# - nums[i+1:]
# - backtrack(arr+[i], nums[i+1:])
# - (back)

# for i in range(n)
#  

class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        combinations = []
        def backtrack(arr, nums):
            if len(arr) == k:
                combinations.append(arr)
                return
            for i in range(len(nums)):
                backtrack(arr + [nums[i]], nums[i+1:])
        
        nums = [i+1 for i in range(n)]
        for i in range(n):
            backtrack([i+1], nums[i+1:])
        return combinations