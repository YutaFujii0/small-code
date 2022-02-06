from typing import List
import heapq

class Solution:
    def call(self, nums: List[int]) -> List[int]:
        evens = []
        odds = []
        ans = []
        for i, val in enumerate(nums):
            if i % 2 == 0:
                heapq.heappush(evens, val)
            else:
                heapq.heappush(odds, val * -1)

        for i in range(len(nums)):
            if i % 2 == 0:
                ans.append(heappop(evens))
            else:
                ans.append(heappop(odds) * -1)
        return ans


print(Solution().call([4,1,2,3]))
print(Solution().call([4,1,2]))
print(Solution().call([2,1]))

