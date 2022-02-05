# property: 
# - frequency of string, order does not matters
# - many times of counting up/down
# - longer is not always better, containing single char may harm till the end

# devide and conquer approach
# devide s by char whose frequency < k
# conquer: max of sub problem
# base case: if string needs no division, return len(s)
# Time complexity: by master theory, T(n) = xT(n/x) + O(n) + O(x)
#  -> a = b^d, nlogn
# Space complexity: by master theory

class Solution:
    def longestSubstring(self, s: str, k: int) -> int:
        freq = Counter(s)
        subComponents = []
        i = 0
        while s and i < len(s):
            if freq[s[i]] < k:
                subComponents.append(s[:i])
                s = s[i+1:]
                i = 0
            else:
                i += 1
        # base case
        if not subComponents:
            return len(s)
        else:
            subComponents.append(s)
        return max([self.longestSubstring(subStr, k) for subStr in subComponents])
