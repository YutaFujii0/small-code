# n = 1, then output is 1
# n = 2, then output is 2
# n = 3, then output is 1,1,1/2,1/1,2 -> 3
# n = 4, then output is 1,1,1,1/1,1,2/1,2,1/2,1,1/2,2 -> 5
# ...oh is this recursive?
# say fn(n) = fn(n-1) + fn(n-2)

# solution:
# if n = 1, return 1
# if n = 2, return 2
# if n > 2, return f(n-1) + f(n-2)
# assurance n > 0
# time complexity: O(1) for n < 3, O(2^(n-2)) for n > 2
# space complexity: O(2^n) because each time fun called
#   another two recursive function is put on stack

# class Solution:
#     def climbStairs(self, n: int) -> int:
#         if n == 1:
#             return 1
#         elif n == 2:
#             return 2
#         else:
#             return self.climbStairs(n - 1) + self.climbStairs(n - 2)

# solution 2:
# way to climb n stairs: 
#  - f(n - 2) ways + 2steps
#  - 2steps + f(n - 2) ways -> no this might be the same as above
#  - 1step + f(n - 2) ways + 1step -> no this might be the same as above

#  - f(n - 2) ways + 2steps
#  - f(n - 2) ways + 1steps + 1steps
#  -> no this miss some case (say, cases where you reach n-1 stair with 2steps)

# what if I solve this recurrence formula
# fn(n) = fn(n-1) + fn(n-2)
# fn(1) = 1
# fn(2) = 2
# fn(n) = 
# x= -(-1) +- sqrt(1 + 4)/ 4*1 = 1 +- sqrt5 / 4
# golden ratio!? no it's fibonnacci
# to be done

# avoid recursion / just use loop
class Solution:
    def climbStairs(self, n: int) -> int:
        if n == 1:
            return 1
        elif n == 2:
            return 2
        else:
            ways = [1, 2]
            for i in range(2, n):
                ways.append(ways[i - 1] + ways[i - 2])

            return ways[n - 1]
