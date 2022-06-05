from typing import List

class Solution:
    def partitionArray(self, nums: List[int], k: int) -> int:
        # take unique
        # sort
        # -> [1,2,4,6,7,10,...]
        # slice every time min-max > k
        # -> let k = 3
        # [1,2,4], [6,7], [10...] we don't even slice but just need to count
        # time complexity: O(n) + O(nlogn) + O(n)
        nums = set(nums)
        nums = list(nums)
        nums.sort()
        left = nums[0]
        right = left
        count = 1
        for num in nums[1:]:
            if num - left > k:
                count += 1
                left = num
        return count

ans = Solution().partitionArray([3,6,1,2,5], 2)
print(ans)
ans = Solution().partitionArray([1,2,3], 1)
print(ans)
ans = Solution().partitionArray([2,2,4,5], 0)
print(ans)