from typing import Optional
# calculate sum using DFS
# Psuedocode
# if head == sum
#  return true
# if head is none
#  return
# left = hasPathSum(head.left, targetSum - head.val)
# right = hasPathSum(head.right, targetSum - head.val)
# return left or right
# Complexity
# - time complexity: O(1) * 2^(O(height)), height = O(logn) thus O(n)
# - space complexity: O(n) for recursion call stack

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False
        if root.left is None and root.right is None and root.val == targetSum:
            return True
        left_tree = self.hasPathSum(root.left, targetSum - root.val)
        right_tree = self.hasPathSum(root.right, targetSum - root.val)
        return left_tree or right_tree
