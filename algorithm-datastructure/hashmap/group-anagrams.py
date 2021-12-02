from typing import List
# like word counting, but not exact the same word
# hash = []
# for word in strs
#  - sort word
#  - hash contains key
#  - add to it
#  - else
#   - create new key [word]
# return hash keys zipped with values
# Complexity:
#  - time complexity: for each iteration, sort O(mlogm) and hash lookup & insert O(1)
#  - total, O(mlogm) where m is the size of letter
#  - space complexity: additional hash, O(n)


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        hash = {}
        for word in strs:
            sorted_word = ''.join(sorted(word))
            if sorted_word in hash.keys():
                hash[sorted_word].append(word)
            else:
                hash[sorted_word] = [word]
        return [hash[key] for key in hash.keys()]
