from typing import List
import heapq

class Solution:
    def kSmallestPairs(self, nums1: List[int], nums2: List[int], k: int) -> List[List[int]]:
        heap = []
        k_elements = []
        i = 0
        j = 0
        for n1 in nums1:
            for n2 in nums2:
                heap.append((n1 + n2, [n1, n2]))
                j += 1
                if i + j > k:
                    break
            i += 1
            j = 0
        return [item[1] for item in nsmallest(k, heap)]
