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

class Solution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        def hash(s, p, m) -> int:
            sumBeforeModulo = 0
            for i, char in enumerate(s):
                sumBeforeModulo += (ord(char) - 96) * pow(p, i, m)
            return sumBeforeModulo % m
        
        for i in range(len(s) - k + 1):
            if hash(s[i:i+k], power, modulo) == hashValue:
                return s[i:i+k]
