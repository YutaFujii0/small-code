class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        m = m-1
        n = n-1
        path = 1
        denominator = 1
        while m:
            path = path * (m + n)
            denominator *= m
            m -= 1

        return path // denominator
