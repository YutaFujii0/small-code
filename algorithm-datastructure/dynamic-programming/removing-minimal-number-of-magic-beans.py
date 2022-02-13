from typing import List

class Solution:
    def minimumRemoval(self, beans: List[int]) -> int:
        def merge_sort(beans):
            if len(beans) < 2:
                return beans
            half = len(beans) // 2
            left = merge_sort(beans[:half])
            right = merge_sort(beans[half:])
            return merge(left, right)
        def merge(left, right):
            ans = []
            while left and right:
                if left[0] > right[0]:
                    ans.append(right.pop(0))
                else:
                    ans.append(left.pop(0))
            if left:
                ans += left
            if right:
                ans += right
            return ans
        m = merge_sort(beans)
        n = len(m)
        costNoRemoval = sum([i - m[0] for i in m])
        dp = [(0,0)] * n
        dp[0] = (costNoRemoval, costNoRemoval) # dummy head
        for i in range(1,n):
            dp[i] = (dp[i-1][0] + m[i-1] - (m[i] - m[i-1]) * (n - i), min(dp[i-1]))
        return min(dp[n-1])


print(Solution().minimumRemoval([4,1,6,5]))
print(Solution().minimumRemoval([2,10,3,2])) #7 11
print(Solution().minimumRemoval([2,10,3,5])) #10
print(Solution().minimumRemoval([3,10,3,5])) #9 (equally 3 beans)
print(Solution().minimumRemoval([3,10,3,5])) #9 (equally 3 beans)
print(Solution().minimumRemoval([100,1])) #1, compare 1 vs 100-1
print(Solution().minimumRemoval([100,1,3])) #4, compare 1+3 vs 100-1 + 3-1 vs 1 + 100-3
print(Solution().minimumRemoval([100,1,50])) #51, compare 1+50 vs 100-1 + 50-1 vs 1 + 100-50
# n = buckets
# if all buckets remain not empty, cost is sum(i - min) := M
# if minimum beans bucket is removed, total potential cost decreases by (next min - previous min) * (n-1)
# heap & greedy approach

# 4,1,6,5 -> n = 4, initial potential cost = 3+0+5+4 = 12
# reduce effect = (4-1) * 3=9, 12-9=3 +  remove cost = 1, total 4, vs 12
# reduce cost = (5-4) * 2=2 3-2=1 + remove cost 4,total 5, vs 3

# greedy?
# dp?
