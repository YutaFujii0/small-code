# k most frequent elements
# 1. count frequency using hashmap
#  - do negation to extract max
# 2. push to heap
# k times loop
#  - poll()
# Complexity:
# time complexity:
#  - counting - O(1) for each iteration, total O(n)
#  - creating heap - usually done with O(n) haepify method
#    but heare I use iteration which need n * O(logn)
#  - extract k smallest: k * O(logn)
# space complexity:
#  - need heap: O(n)
#  - need k frequent values store: trivial

from typing import List
import heapq

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count_map = {}
        for num in nums:
            if num in count_map.keys():
                count_map[num] -= 1
            else:
                count_map[num] = -1

        h = []
        k_frequent = []
        for item in [(count_map[k], k) for k in count_map]:
            heappush(h, item)
        # heap = heapify([(count_map[k], k) for k in count_map])
        for i in range(k):
            k_frequent.append(heappop(h)[1])
        return k_frequent




# ----------------------------
#  Explained Solution
# ----------------------------
# from collections import Counter
# class Solution:
#     def topKFrequent(self, nums: List[int], k: int) -> List[int]:
#         # O(1) time
#         if k == len(nums):
#             return nums
#
#         # 1. build hash map : character and how often it appears
#         # O(N) time
#         count = Counter(nums)
#         # 2-3. build heap of top k frequent elements and
#         # convert it into an output array
#         # O(N log k) time
#         return heapq.nlargest(k, count.keys(), key=count.get)
