# idea: recursion
# Psuedocode
# localMax = nums[-1]
# k = n - 1
# while k >= 0
#  if nums[k] >= localMax (include equal to)
#    (move k back to next)
#    (and make nums after index k sorted order)
#    - localMax = nums[k].copy()
#    - pop(k) and push_back
#    - k--
#  else (:swap)
#    - find next smallest value to nums[k]
#    - swap[k] and that value

class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        localMax = nums[-1]
        k = len(nums) - 2
        while k >= 0:
            if nums[k] >= localMax:
                tmp = nums[k]
                localMax = tmp
                nums.append(nums.pop(k))
                k -= 1
            else:
                i = k + 1
                while nums[i] <= nums[k]:
                    i += 1
                tmp = nums[k]
                nums[k] = nums[i]
                nums[i] = tmp
                return
