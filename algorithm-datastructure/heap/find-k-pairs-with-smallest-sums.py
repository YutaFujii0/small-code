from typing import List
import heapq

# Using heap
# Idea: push candidates into heap, lookup min each time
# Always next smallest pair exist 'near' previous (i,j) pair
# Set heap, explored
# k = min of k and multiple of length
# while k
#  - i,j = heap pop
#  - heap push (i+1,j) if not explored
#  - heap push (i,j+1) if not explored
#  - k --
# Complexity
#  - time complexity: O(3logn) for each iteration, k times
#  - thus O(3k * logn)
#  - space complexity: need an extra set of (i,j), thus O(n^2)

class Solution:
    def kSmallestPairs(self, nums1: List[int], nums2: List[int], k: int) -> List[List[int]]:
        heap = [(nums1[0] + nums2[0], 0, 0)]
        k_elements = []
        explored = set()
        k = min([k, len(nums1) * len(nums2)])
        while k:
            min_value, i, j = heapq.heappop(heap)
            explored.add((i, j))
            k_elements.append([nums1[i], nums2[j]])
            k -= 1

            candidate1 = (nums1[i + 1] + nums2[j], i + 1, j) if i + 1 < len(nums1) else None
            candidate2 = (nums1[i] + nums2[j + 1], i, j + 1) if j + 1 < len(nums2) else None

            if candidate1 and not (i + 1, j) in explored:
                heapq.heappush(heap, candidate1)
                explored.add((i + 1, j))
            if candidate2 and not (i, j + 1) in explored:
                heapq.heappush(heap, candidate2)
                explored.add((i, j + 1))

        return k_elements



# naive solution

# class Solution:
#     def kSmallestPairs(self, nums1: List[int], nums2: List[int], k: int) -> List[List[int]]:
#         heap = []
#         k_elements = []
#         i = 0
#         j = 0
#         for n1 in nums1:
#             for n2 in nums2:
#                 heap.append((n1 + n2, [n1, n2]))
#                 j += 1
#                 if i + j > k:
#                     break
#             i += 1
#             j = 0
#         return [item[1] for item in nsmallest(k, heap)]
