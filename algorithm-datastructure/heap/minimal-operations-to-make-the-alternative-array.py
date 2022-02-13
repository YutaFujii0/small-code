from typing import List
import collections

# take most frequent two numbers from odd , even index elemenents
# if largest of odd/even is not the same number, 
# else compare 1,2 count

# Complexity
# Time complexity O(nlogn)
# 2 * O(n) for odd/even elements generation +
# O(n) for heapify and 2 * O(logn) for heap pop + 
# O(1) for conditional calculation
# Space complexity: 

class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        if len(nums) == 1: return 0
        oddsCount = collections.Counter([v for index, v in enumerate(nums) if index % 2 == 0])
        evensCount = collections.Counter([v for index, v in enumerate(nums) if index % 2 == 1])
        oddFrequent = oddsCount.most_common()[:2]
        evenFrequent = evensCount.most_common()[:2]
        if oddFrequent[0][0] != evenFrequent[0][0]:
            return len(nums) - oddFrequent[0][1] - evenFrequent[0][1]
        else:
            n = len(oddFrequent)
            m = len(evenFrequent)
            # if either part contains only single number, 
            # forcefully the other side have to use 2nd most frequent number
            if n == 1 and m == 1:
                return len(nums) - max(oddFrequent[0][1], evenFrequent[0][1])
            elif n == 1:
                return len(nums) - oddFrequent[0][1] - evenFrequent[1][1]
            elif m == 1:
                return len(nums) - oddFrequent[1][1] - evenFrequent[0][1]
            if (oddFrequent[0][1] - oddFrequent[1][1]) > (evenFrequent[0][1] - evenFrequent[1][1]):
                # high cost to use most frequent of even element
                return len(nums) - oddFrequent[0][1] - evenFrequent[1][1]
            else:
                return len(nums) - oddFrequent[1][1] - evenFrequent[0][1]


print(Solution().minimumOperations([3,1,3,2,4,3]))
print(Solution().minimumOperations([1,2,2,2,2]))
print(Solution().minimumOperations([1,2]))
print(Solution().minimumOperations([1]))
print(Solution().minimumOperations([2,2,2,2,2]))
print(Solution().minimumOperations([1,2,2,2,2,3,4,5,6]))
