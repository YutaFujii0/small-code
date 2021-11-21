# return its sum, not subarray itself
# subarray must have at least one element
# possible solution
# given an array of 6 elements, possible subarray contains 1 to 6 elements
# - for 1 element subarray, 6 candidates
# - for 2 element subarray, 5 candidates
# - for 3 element subarray, 4 candidates
# - for 4 element subarray, 3 candidates
# - for 5 element subarray, 2 candidates
# - for 6 element subarray, 1 candidates
# total 21 candidates( O(n^2) time complexity)
# among those choose max. (keep max sum/value in iteration: O(1) space complexity)

# 4,3,2,-100,1,5,6 -> two banks what should i do
# iterate through left to right and
# accumulate, 
# :
#  - accumulate sum > 0, keep
#  - accumulate sum < 0, push sum into final candidates and reset accumulated sum


# [-2,1,-3,4,-1,2,1,-5,4] -> 6 [4,-1,2,1]
# [-2,1,-3,4,-1,2,1,-5,4,4] -> 9 [4,-1,2,1,-5,4,4]
# [-2,1,-3,4,-1,2,1,-5,100] -> 101 [4,-1,2,1,-5,100]
# [-2,1,-3,4,-1,2,1,-5,4,-5,4,-5,4,100] -> 104 [4,100]
# [-2,1,-3,4,-1,2,1,-5,4,-5,4,100] -> 104 [4,100]
# [-2,1,-3,4,-1,2,1,-5,4,-5,4,1] -> 6 [4,-1,2,1]

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]

        maximals = []
        accum = None
        for i in nums:
            if accum == None:
                if i > 0:
                    accum = i
                else:
                    maximals.append(i)
            else:
                if i < 0:
                    maximals.append(accum)
                if accum + i <= 0:
                    maximals.append(accum)
                    accum = None
                else:
                    accum += i
        if accum != None:
            maximals.append(accum)
        
        max = maximals[0]
        for val in maximals:
            if max < val:
                max = val
        
        return max
                