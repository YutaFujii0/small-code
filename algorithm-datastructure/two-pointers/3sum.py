from typing import List

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        def merge_sort(nums) -> List[int]:
            if len(nums) < 2:
                return nums
            half = len(nums) // 2
            left = merge_sort(nums[:half])
            right = merge_sort(nums[half:])
            return merge(left, right)
        def merge(left, right) -> List[int]:
            merged = []
            while left and right:
                if left[0] > right[0]:
                    merged.append(right.pop(0))
                else:
                    merged.append(left.pop(0))
            if left:
                merged += left
            if right:
                merged += right
            return merged

        n = len(nums)
        if n < 3:
            return []
        nums = merge_sort(nums)
        ans = []
        for i in range(0, n-2):
            if i == 0 or nums[i-1] != nums[i]:
                target = nums[i] * -1
                l = i+1
                r = n-1
                while l < r:
                    if nums[l] + nums[r] == target:
                        ans.append([nums[l], nums[i], nums[r]])
                        while nums[r-1] == nums[r] and l < r:
                            r -= 1
                        r -= 1
                        while nums[l] == nums[l+1] and l < r:
                            l += 1
                        l += 1
                    elif nums[l] + nums[r] > target:
                        r -= 1
                    else:
                        l += 1
        return ans
