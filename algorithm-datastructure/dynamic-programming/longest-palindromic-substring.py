# expample "abacedeffedecabxyz" -> "bacedeffedecab"

# property of the problem: need to seek valid palindrome for each head string
# idea: dynamic programming?
# - if we know the answer of palindrome of which head is ith, 
# can we induce i+1th palindrome easily? if possible, O(n) solution
# idea: sliding window?

# idea: brute force
# for i in n, check palindrome whose head is ith string
# O(n) * n solution
# feels like this has so many duplicate process...dp may fit?

# in the first place, 
# P: find longest palindrome starting head char? how?

# three pointer? head, middle, tail
# 

class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        dp = [[False] * n] * n
        self.longestPalindrome = (1, s[0])
        def palindrome(i,j):
            while True:
                if i > 0 and j < n-1 and s[i-1] == s[j+1]:        
                    i -= 1
                    j += 1
                    dp[i][i] = True
                else:
                    break
            if j-i+1 > self.longestPalindrome[0]:
                self.longestPalindrome = (j-i+1, s[i:j+1])
        for i in range(n):
            dp[i][i] = True
            if i > 0 and s[i-1] == s[i]:
                dp[i-1][i] = True
                palindrome(i-1,i)
        for i in range(n):
            palindrome(i,i)
        return self.longestPalindrome[1]
