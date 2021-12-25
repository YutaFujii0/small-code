from typing import List
# Psuedocode
# low = 0
# hight = len(nums) -1
# while low < high
# mid = gauss number of (low + hight)/2
# if target < mid
#  - cut higher half by setting hight = mid-1
# else target > mid
#  - cut lower half by setting low = mid+1
# return high +1

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        low = 0
        high = len(nums) - 1
        while low <= high:
            mid = (low + high) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] > target:
                high = mid - 1
            else:
                low = mid + 1
        return high + 1
