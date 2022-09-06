# nums = [-1,2,1,-4], target = 1
# possible 3 pairs = 4C3 = 4
# -1,2,1: 2 closest
# -1,2,-4: -3
# -1,1,-4: -4
# 2,1,-4: -1

# naive solution:
# - calculate sum of all possible 3-sets
# - take diffs of those sums vs target, and make them absolute values
# - take min of those diff
# Time complexity: nC3 = O(n^3) + O(n) + O(n) ~ O(n^3)
# Space complexity: O(n)

# 3sum approach
# O(n^2)
# if I fail to find 3 nums whose sum == target, 
#  - decrement & increment target and start again

# Sorting and scan
# - sort nums
# for i in num(nums)
#  - fix nums[i] as an element
# (find 2 sum that is closest to target - nums[i])
#  - left = i+1, right = n-1
#  if nums[left] + nums[right] > target:
#    right -= 1
#  else if the opposite:
#    left += 1
#  else:
#    stop iterating and return
#  - take min
# - take min
# Time complexity: O(nlogn) + n * (O(n) + O(1)) ~ O(n^2)
# Space complexity: O(n)

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        n = len(nums)
        nums.sort()
        
        closestSum = 10 ** 5
        diff = abs(closestSum - target)
        for i in range(n - 2):
            left = i + 1
            right = n - 1
            
            while left < right:
                # compare if current 3 sum is the closer
                threeSum = nums[i] + nums[left] + nums[right]
                if abs(threeSum - target) < diff:
                    closestSum = threeSum
                    diff = abs(threeSum - target)
                if threeSum > target:
                    right -= 1
                elif threeSum < target:
                    left += 1
                else:
                    return target

            
        return closestSum
