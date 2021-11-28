from typing import Optional

# basically searching from smallest value and count up
# using recursion
# Psuedocode
# set count = 0
# set values = []
# if left node is not null
#  - recurse(left node)
# - push val of node into values
# - count ++
# - recurse(right node)
# Complexity
# time complexity: O(1) * k
# space complexity: O(1) * k (for recursion)

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        self.count = 0
        self.kth_val = None
        self.target = k

        self.traverse(root)
        return self.kth_val

    def traverse(self, head: TreeNode):
        if head.left:
            self.traverse(head.left)
        self.count += 1
        if self.count == self.target:
            self.kth_val = head.val
            return
        if head.right:
            self.traverse(head.right)
