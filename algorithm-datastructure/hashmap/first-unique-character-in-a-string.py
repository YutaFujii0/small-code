# count(lookup heavy) and filter count==1
# or push into array and remove if cache hits

# hashmap = {}
# for char in s
#  count ++
# for char, index in s
#  if count == 1
#   return index
# return -1
# Complexity
#  - time complexity: O(1)inserting * n + O(1) lookup * n(worst case)
#  - space complexity: O(n) for hashmap

class Solution:
    def firstUniqChar(self, s: str) -> int:
        counts = {}
        for char in s:
            if char in counts:
                counts[char] += 1
            else:
                counts[char] = 1
        for index, char in enumerate(s):
            if counts[char] == 1:
                return index
        return -1
