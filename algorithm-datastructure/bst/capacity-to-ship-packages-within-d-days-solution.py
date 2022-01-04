import itertools

# The idea is to try a bunch of different max-capacities starting from the lower bound which is the maximum of all weights up until the sum of all the weights.

# The lower bound is the max weight because of the following: say the max weight were W. If our capacity were W - 1, we could never ship the item.

# The upper bound can be simply the sum of the weights as that would be the capacity needed to ship in a single day.

# Anyway, each time we evaluate a capacity, we see whether we can greedily ship all the weights within the allotted time. If so, it's successful.

# Consider the responses for each capacity check as a 1 or 0 (it can ship within d days using capacity c or it cannot).

# So, our answer is essentially the capacity corresponding to the right-most 1 in a boolean list of capacity evaluations .... [0,0,0,0,0,1,1,1,1,1,1...]

class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        def search(capacity: int) -> bool:
            i = 0
            for d in range(days):
                total = 0

                while i < len(weights) and total + weights[i] <= capacity:
                    total += weights[i]
                    i += 1

            return i == len(weights)
        
        left = max(weights)
        right = sum(weights)
        
        while left <= right:
            median = (left + right) // 2
            value = search(median)
            if not value:
                left = median + 1
            else:
                right = median - 1
        
        return left
