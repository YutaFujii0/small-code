from typing import List
# step 1. create (start, end) (inclusive) array that meets target
# step 2. dp
#  - dp1 min length left of i
#  - dp2 min length right of i
# step 3. take min of dp1[i] + dp2[i]

class Solution:
    def minSumOfLengths(self, arr: List[int], target: int) -> int:
        n = len(arr)
        intMax = 1000000

        for i in range(1, n):
            arr[i] = arr[i-1] + arr[i]

        arr = [0] + arr
        left = 0
        right = 0        
        subArrays = []
        while left < n+1 and right < n+1:
            if arr[right] - arr[left] == target:
                subArrays.append((left, right-1))
                left += 1
            elif arr[right] - arr[left] > target:
                left += 1
            else:
                right += 1
        # [(0,0),(4,4)]
        # [(0,0), (1,2), (4,4)]
        # [(0,2), (1,3), (2,4)] <- [1,1,1,1,1] target 3
        
        dp1 = [intMax] * n
        dp2 = [intMax] * (n+1)
        m = len(subArrays)
        x = 0
        for i in range(n):
            length = intMax
            while x < m and subArrays[x][1] == i:
                suffix = subArrays[x][1]
                length = subArrays[x][1] - subArrays[x][0] + 1
                dp1[i] = min(dp1[i-1], length)
                x += 1
            dp1[i] = min(dp1[i-1], length)

        x = m - 1
        for i in range(n-1, -1, -1):
            length = intMax
            while 0 <= x and subArrays[x][0] == i:
                prefix = subArrays[x][0]
                length = subArrays[x][1] - subArrays[x][0] + 1
                dp2[i] = min(dp2[i+1], length)
                x -= 1
            dp2[i] = min(dp2[i+1], length)

        res = intMax
        for i in range(n-1):
            res = min(res, dp1[i] + dp2[i+1])
        return -1 if res == intMax else res
