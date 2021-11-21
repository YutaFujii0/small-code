class Solution:
    def checkAlmostEquivalent(self, word1: str, word2: str) -> bool:
        counts = {}
        for s in word1:
            if s in counts:
                counts[s] += 1
            else:
                counts[s] = 1
        
        for s2 in word2:
            if s2 in counts:
                counts[s2] -= 1
            else:
                counts[s2] = -1
        
        for k in counts:
            if 3 < counts[k] or counts[k] < -3:
                return False
        
        return True
