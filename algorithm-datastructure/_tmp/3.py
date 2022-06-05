from typing import List

class Solution:
    def arrayChange(self, nums: List[int], operations: List[List[int]]) -> List[int]:
        # hashmap hold conversion
        # { 1 -> 3, 2 -> 4, ...}
        # for each operations, 
        # if [i][0] not in hashmap, set this as key
        # else, update value of the key[i][0] to [i][1]
        # for each num
        # - look up hashmap and update its valuet to the value of the key
        # - remain the same if key not exist
        conversion = {}
        tails = {}
        for op in operations:
            if op[0] in tails:
                head = tails[op[0]]
                conversion[head] = op[1]
                tails[op[1]] = head
            else:
                conversion[op[0]] = op[1]
                tails[op[1]] = op[0]
        for i in range(len(nums)):
            if nums[i] in conversion:
                nums[i] = conversion[nums[i]]

        return nums

ans = Solution().arrayChange([1], [[1,2],[2,3],[3,1000000],[1000000,1]])
print(ans)

ans = Solution().arrayChange([1,2], [[1,3],[2,1],[3,2]])
print(ans)

ans = Solution().arrayChange([1,2,4,6], [[1,3],[4,7],[6,1]])
print(ans)

