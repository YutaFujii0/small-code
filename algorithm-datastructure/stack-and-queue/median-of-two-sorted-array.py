from typing import List
# property: two array are sorted, and log(m+n) time complexity is required
# idea: treat nums like queue
# psuedocode
# m, n = len(nums1) len(nums2)
# median1, 2 = (math.floor(m+n-1), math.ceil(m+n-1))
# i = 0
# while i < median1
#   if num1[0] < num2[0], pop num1
#   else pop num2
# m1 = 
# m2 = 
# return median1 + median2 / 2.0
# Complexity:
# time complexity: O(m+n) at worst
# space complexity: O(1)

class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        m, n = len(nums1), len(nums2)
        odd = (m+n) % 2 == 1
        medianIndex = (m+n-1) // 2
        i = 0
        def minimumPop() -> int:
            if not nums1:
                return nums2.pop(0)
            if not nums2:
                return nums1.pop(0)
            if nums1[0] < nums2[0]:
                return nums1.pop(0)
            else:
                return nums2.pop(0)
        while i < medianIndex:
            minimumPop()
            i += 1
        if odd:
            return float(minimumPop())
        else:
            return (minimumPop() + minimumPop()) / 2.0
