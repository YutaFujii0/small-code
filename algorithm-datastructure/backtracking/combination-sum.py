from typing import List
# idea: backtracking
# set tmpCandidates as array and push candidates 
# failure condition: its sum suppress target
# success condition: its sum equals to target

# Psuedocode
# results = []
# startIndex = 0
# for i in range(startIndex):
#   - backtrack(0, [])

# def backtracking(startIndex, tmpCandidates)
#   - if condition is met:
#     - results.append(tmpCandidates)
#   - for j in range(startIndex, n)
#     - tmpCandidates.append(nums[j])
#     - backtrack(j, tmpCandidates)
#     - tmpCandidates.pop()
# return results
# import pdb

class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        def backtrack(startIndex: int, tmpCandidates: List[int], target: int):
            if target < 0:
                return
            elif target == 0:
                results.append(tmpCandidates[:])
                return
            for j in range(startIndex, n):
                tmpCandidates.append(candidates[j])
                backtrack(j, tmpCandidates, target - candidates[j])
                tmpCandidates.pop()
        results = []
        n = len(candidates)
        backtrack(0, [], target)
        return results


nums = [2,3,5,7]
target = 7
print(Solution().combinationSum(nums, target))
