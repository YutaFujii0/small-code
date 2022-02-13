class Solution:
    def call(self, num1: int, num2: int) -> int:
        i = 0
        while num1 > 0 and num2 > 0:
            if num1 >= num2:
                num1 -= num2
            else:
                num2 -= num1
            i += 1
        return i

print(Solution().call(10, 10))
print(Solution().call(100000, 10))
print(Solution().call(100000, 1))
print(Solution().call(2, 3))
print(Solution().call(2, 0))
print(Solution().call(0, 0))
