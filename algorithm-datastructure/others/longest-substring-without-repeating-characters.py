# naive solution:
# for each char, examine the longest substring
# then take the max
# -> n * O(n^2) + O(n) ~ O(n^3)

# idea: sliding window
# each time repetition detected, reset window
# max of the window size is the answer
# Psuedocode
# left=0, right=0
# max_window = 0
# last_position = {}
# while right < length of s
#  - if last_position[right] >= left (repetition detected!)
#    - max_window = max([max_window, window size])
#    - left = last_position[right]+1
#  - insert s[right] into last_position
#  - right++

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        left = 0
        right = 0
        maxWindow = 0
        lastAppearedAt = {}
        while right < len(s):
            if s[right] in lastAppearedAt and lastAppearedAt[s[right]] >= left:
                maxWindow = max([maxWindow, right - left])
                left = lastAppearedAt[s[right]] + 1
            lastAppearedAt[s[right]] = right
            right += 1
        return max([maxWindow, right - left])
