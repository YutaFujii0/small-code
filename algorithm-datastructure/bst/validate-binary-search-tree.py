from typing import Optional
# using recursion
# validate left node
# validate right node
# validate max of left node < root.val < min of right node
# Complexity
# - time complexity: O(1) for conqur step, plus recursion O(n)
# - space complexity: O(n) recursion calls

# BFS approach
# each layer ... no it does not assure binary search tree...

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def isEdge(node: TreeNode) -> bool:
            return node is None or (node.left is None and node.right is None)

        def validMinMax(root):
            if root is None:
                return (True, None, None)

            if isEdge(root):
                return (True, root.val, root.val)

            left = root.left
            right = root.right
            (left_valid, left_min, left_max) = validMinMax(left)
            if not left_valid: return (False, None, None)
            (right_valid, right_min, right_max) = validMinMax(right)
            if not right_valid: return (False, None, None)
            left_valid = left_max is None or left_max < root.val
            right_valid = right_min is None or root.val < right_min
            return (left_valid and right_valid, left_min or root.val, right_max or root.val)
        return validMinMax(root)[0]
