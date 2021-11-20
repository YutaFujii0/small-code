from typing import List

# Use hash table to store frequency counts
# scan arr
#   push hash table count
# for i in hash keys
#   skip if value > 1
#   when k th element, return

# Time complexity:
#  - n iterations * O(1) for store counting numbers,
#  - k(<n) iterations for checking if count is 1(which is O(1))
#  - total: n + k ~ O(n)
# Space complexity:
#  - need an extra hashmap, storing at most n items
#  - O(n)
class Solution:
    def kthDistinct(self, arr: List[str], k: int) -> str:
        counts = {}
        for s in arr:
            if s in counts:
                counts[s] += 1
            else:
                counts[s] = 1

        i = 1

        for key in counts:
            if counts[key] == 1:
                if i == k:
                    return key
                else:
                    i += 1

        return ""
