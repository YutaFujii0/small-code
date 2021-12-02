from typing import List
# Intuition: heavy lookup(if element is already appeared)
# -> Hashmap
# appeared = {}
# for num in nums1
#  - insert into hash
# for num in nums2
#  - lookup
#  - if exists, append to return array
# return array
# Complexity:
#  time complexity: insertion and lookup O(1) * n
#  space complexity: additional hash table O(n)

class Solution:
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        intersection = {}
        appeared = {}
        for num in nums1:
            appeared[num] = True
        for num in nums2:
            if num in appeared:
                intersection[num] = True
        return intersection.keys()
# practical solutions(not good as study)

# class Solution:
#     def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
#         return set(nums1) & set(nums2)

# class Solution:
#     def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
#         intersection = set()
#         appeared = set()
#         for num in nums1:
#             appeared.add(num)
#         for num in nums2:
#             if num in appeared:
#                 intersection.add(num)
#         return intersection
