from typing import List

class Solution:
    def minMaxGame(self, nums: List[int]) -> int:
        # n = len(nums)
        # while n > 1
        # - newNum = []
        # - every 2 pair with index i, take
        #  - min if i is odd
        #  - max if i is even
        # - nums = newNums
        # return the only element of nums
        n = len(nums)
        while n > 1:
            newNums = []
            for i in range(n // 2):
                el = min(nums[2 * i], nums[2 * i + 1]) if i % 2 == 0 else max(nums[2 * i], nums[2 * i + 1])
                newNums.append(el)
            nums = newNums
            n //= 2
        return nums[0]

ans = Solution().minMaxGame([1,3,5,2,4,8,2,2])
print(ans)
ans = Solution().minMaxGame([3])
print(ans)
ans = Solution().minMaxGame([3,2])
print(ans)
ans = Solution().minMaxGame([3,5,1,4])
print(ans)