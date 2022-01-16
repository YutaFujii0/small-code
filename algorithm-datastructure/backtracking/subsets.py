from typing import List

class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        def backtrack(first = 0, curr = []):
            # if the combination is done
            if len(curr) == k:  
                output.append(curr[:])
                return
            for i in range(first, n):
                # add nums[i] into the current combination
                curr.append(nums[i])
                # use next integers to complete the combination
                backtrack(i + 1, curr)
                # backtrack
                curr.pop()

        output = []
        n = len(nums)
        for k in range(n + 1):
            backtrack()
        return output

class Solution2:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        results = [[]]
        for num in nums:
            results += [items + [num] for items in results]
        return results

class Solution3:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        results = []
        n = len(nums)
        for i in range(2**n, 2**(n+1)):
            bitmask = bin(i)[3:]
            child = [nums[i] for i in range(n) if bitmask[i] == '1']
            results.append(child)
        return results
