# property: existence of substring
# ex) p = ".*abc.*xyz", then 
# "abcdddxyzwwwxyz" is a valid match, but I need to check both
# "xyz" in the middle and "xyz" in the end

# idea: bruteforce
# every time * comes, check next fixed substring
# "abcd*efg" -> look for all efg

# idea: treat like a graph of chunk of strings? 'abc' -> ... -> 'xy'
# ...no good solution

# recursion, backtracking
# backtrack(s[i:], p[j:])
# stop condition: 
#  [success] s[i:] reaches to an end, p[j:] as well
#  [failure] s[i] != p[j] (true if p[j] is '.')
# 
#  if p[j] == "*"
#  - backtrack(s[i+1:], p[j+1])
#  else
#  - backtrack(s[i+1:], p[j+1])
# back

class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        match = {}
        def dp(i, j):
            if (i, j) in match:
                return match[(i, j)]
            if j == len(p):
                result = i == len(s)
            else:
                matchFirstLetter = i < len(s) and p[j] in { s[i], "." }
                if j+1 < len(p) and p[j+1] == "*":
                    result = dp(i, j+2) or (matchFirstLetter and dp(i+1, j))
                else:
                    result = matchFirstLetter and dp(i+1, j+1)
            match[(i, j)] = result
            return result
        return dp(0, 0)
