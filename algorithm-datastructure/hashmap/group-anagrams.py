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


# Anagram words have in common counts of each letters
# i.e) eat and tea has a:1, e:1, t:1
# convert string into character count
# look up if the same set exists, and group them
# Hashtable
# hash = {}
# for str in strs
#  - convert str to char-count
#  - add to hash table
# for item in hashtable
#  - push value to return array
# return array
# Complexity:
# - time complexity: n * O(1) + O(keys)
# - space complexity: O(n) for hashtable (or even more to use hashtable)



class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        alphabet_composition = {}
        for word in strs:
            key = [0]*26
            for char in word:
                key[ord(char) - 97] += 1
            alphabet_composition.setdefault(tuple(key), []).append(word)
        return [value for _, value in alphabet_composition.items()]
