# https://leetcode.com/problems/longest-common-prefix/
# strs has at least one string(if that is the case, that string is the answer)
# there's no assurrance strs are in alphabetical order
# given the sorted strs,  can I do this easily? no it doesn't matter
# because what we need to do is answer prefix all elements in strs have in common


# solution
# sort elements: time complexity O(NlogN), space complexitiy: O(NlogN) since we use recurrsive calls
# compare first and last ones and take prefix both have in common
for i in range(len(s)):
    if s[i] in ["(", "[", "{"]:
        stack.append(s[i])
    elif s[i] in pop_map:
        if stack.pop() != pop_map[s[i]]:
            print("false")
import math

class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        strs_sorted = self.merge_sort(strs)
        first = strs_sorted[0]
        last = strs_sorted[-1]
        # prefix that first and last have in common
        prefix = ""
        for i in range(len(first)):
            if first[0:i+1] == last[0:i+1]:
                prefix = first[0:i+1]
        return prefix


    def merge_sort(self, arr: List[str]) -> List[str]:
        if len(arr) < 2:
            return arr

        left = []
        right = []
        for i in range(math.ceil(len(arr) / 2)):
            # import pdb; pdb.set_trace()
            left.append(arr.pop(0))
        right = arr
        return self.merge(self.merge_sort(left), self.merge_sort(right))

    def merge(self, left: List[str], right: List[str]) -> List[str]:
        result = []
        while len(left) > 0 and len(right) > 0:
            if left[0] < right[0]:
                result.append(left.pop(0))
            else:
                result.append(right.pop(0))

        while len(left) > 0:
            result.append(left.pop(0))
        while len(right) > 0:
            result.append(right.pop(0))

        return result
