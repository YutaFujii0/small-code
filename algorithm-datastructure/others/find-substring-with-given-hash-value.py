# property: lookup substring that meets criteria, naively O(n^2) lookups
# BUT: constrain k length reduces possible candidates to only n - k + 1
# also implement hash function first
# hash function
# apply modulo as many as possible to prevent overflow
# Psuedocode
# for i in 0..n-k+1
#  if hash(s[i:i+k], p, m) == hashValue
#   - return s[i:i+k]
# (no default case given answer do exists)
# Time Complexity: O(k) for hash function * (n - k + 1) ~ O(nk)

class NaiveSolution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        def hash(s, p, m) -> int:
            sumBeforeModulo = 0
            for i, char in enumerate(s):
                sumBeforeModulo += (ord(char) - 96) * pow(p, i, m)
            return sumBeforeModulo % m
        
        for i in range(len(s) - k + 1):
            if hash(s[i:i+k], power, modulo) == hashValue:
                return s[i:i+k]



# UPDATE
# idea: sliding window
# for the heavy part is calculating hash
# window [i:i+k]
#      hash(i)   =                      s_i * p^1  +  s_i+1 * p^2  +  ...  +  s_i+k-1 * p^(k-2)  +  s_i+k * p^(k-1)
# window [i-1:i+k-1]
#      hash(i-1) =        s_i-1 * p  +  s_i * p^2  +  s_i+1 * p^3  +  ...  +  s_i+k-1 * p^(k-1)
# difference
#      hash(i-1) = hash(i) * p  +  s_i-1 * p  -  s_i+k * p^k (not k-1 but k)

class Solution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        n = len(s)
        curValue = 0
        for i, j in enumerate(range(n-k, n)):
            curValue += (ord(s[j]) - 96) * pow(power, i, modulo)
        
        curValue %= modulo
        i = n - k
        hit = 0
        while i >= 0:
            if curValue == hashValue:
               hit = i
            i -= 1
            curValue = (curValue * power + (ord(s[i]) - 96) - (ord(s[i+k]) - 96) * pow(power, k, modulo)) % modulo

        return s[hit:hit+k]

print(Solution().subStrHash("leetcode", 7, 20, 2, 0))
print(Solution().subStrHash("fbxzaad", 31, 100, 3, 32))
