from typing import List
# idea: binary search like brute force
# possible answer
# lower possible answer = (total weight) / days
# higher possible answer = (total weight)

# Psuedocode
# high = sum(weight)
# low = high // days
# while low <= high
# - mid = (high + low) / 2
# - i = 0, j = 0, sum = 0
# - while j < days
#  - if mid - sum >= weights[i]
#   - sum += weights[i]
#   - i++
#  - else
#   - sum = 0
#   - j++
# - if i == length(weights) - 1 (packaging success, answer can be lower)
#  - high = mid - 1
# - else (packaging failure, threthold should be higher)
#  - low = mid + 1
# return low

# Complexity:
# - time complexity: logW where W is total sum of weights
# - space complexity: high,low,i,day,dailySum are all O(1) space needed

class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        volume = len(weights)
        high = sum(weights)
        low = high // days
        while low <= high:
            mid = (high + low) // 2
            i = 0
            day = 1
            dailySum = 0
            while day <= days and i < volume:
                dailySum += weights[i]
                if mid < dailySum:
                    dailySum = 0
                    day += 1
                    continue
                i += 1
            if i == volume:
                high = mid - 1
            else:
                low = mid + 1
        return low
