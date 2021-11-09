# non-decreasing order means it's sorted
# they never increase at any step
# num.length can be zero
# each element can be negative
# must not use an extra space, calculate with O(1) space
# Return k after placing the final result in the first k slots of nums.


# solution
# - current_val = None
# - for i in nums
#   - if current_val != i
#   - current_val = i
#   - else pop i
# - return len(nums)


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        current_val = None
        current_index = 0
        for i in range(len(nums)):
            if current_val != nums[current_index]:
                current_val = nums[current_index]
                current_index += 1
            else:
                nums.pop(current_index)
        
        return len(nums)
