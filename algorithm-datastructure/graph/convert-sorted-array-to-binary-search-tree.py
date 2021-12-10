from typing import Optional, List
# like binary search, pick center value to node, and recuese it
# i = len(nums) // 2
# head = nums[i]
# left = nums[:i]
# right = nums[i+1:](or out of index)
# head.left = sortedarray(left)
# head.right = sortedarray(right)
# Complexity
# - time complexity: O(n)
# - space complexity: O(n) for recursion

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        if not nums:
            return None
        i = len(nums) // 2
        head = nums[i]
        left = nums[:i]
        right = nums[i+1:] if i+1 < len(nums) else []
        return TreeNode(head, self.sortedArrayToBST(left), self.sortedArrayToBST(right))
