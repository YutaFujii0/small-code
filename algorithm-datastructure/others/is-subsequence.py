# idea:pointer moving
# set i - 0 as s pointer, 
# iterate through char in t and increment i every time s[i]==t[j]
# after iteration or the middle, if i reaches len(s), then true

# Complexity
# - Time complexity: n * O(1) ~ O(n)
# - space complexity: constant O(1)


class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        i = 0
        n = len(s)
        if n == 0:
            return True
        for j in range(len(t)):
            if s[i] == t[j]:
                i += 1
                if i == n:
                    return True
        return i == n