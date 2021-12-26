from typing import List
# Idea: find kth index where nums[k] > nums[k+1] call this (*)
# set i at the medium of the array, if nums[i] > nums[0], this means (*) lies on right half
# if nums[i] < num[0], this means (*) lies on left half

# Psuedocode
# low=0, high=n-1
# while low <= high
# - mid = gauss number of (low + high)/2
# - if nums[mid] > nums[mid+1](FOUND!!)
#  - return nums[mid+1]
# - if nums[0] < nums[mid]
#  - low=mid+1
# - else
#  - high = mid-1
# edge case: nums are actually sorted array: nums[0] < nums[n-1]
# - return nums[0]
# Complexity:
# - time complexity: master method, T(n) = 1*T(n/2) + O(1) -> a < b^d  n^dlogn

class Solution:
    def findMin(self, nums: List[int]) -> int:
        if nums[0] < nums[-1] or len(nums) == 1:
            return nums[0]
        low = 0
        high = len(nums) - 1
        while low <= high:
            mid = (low + high) // 2
            if nums[mid] > nums[mid+1]:
                return nums[mid+1]
            if nums[0] <= nums[mid]:
                low = mid + 1
            else:
                high = mid - 1
        return nums[high]
