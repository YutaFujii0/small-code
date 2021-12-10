from typing import Optional
# traverse tree until all 'overwrapped tree' node explored
# using BFS/DFS

# Psuedocode(DFS, recursion)
# head = head1
# dfs(head1, head2)
# return head

# def dfs(head1, head2)
# head = head1
# if head is none
#  - return 
# - if left1 and left2
#   - recursion on left
#  - else left1 or left2
#   - connect nodes to head's left
#  - sum head
#  - if right1 and right2
#   - recursion on right
#  - else right1 or right2
#   - connect nodes to head's left
# Complexity
# - time complexity: O(n) (more precisely, minimum tree nodes)
# - space complexity: O(n) (more precisely, minimum tree nodes) for recursion

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(head1: Optional[TreeNode], head2: Optional[TreeNode]):
            if head1 is None or head2 is None:
                return
            if head1.left and head2.left:
                dfs(head1.left, head2.left)
            elif head2.left:
                head1.left = head2.left
            head1.val += head2.val
            if head1.right and head2.right:
                dfs(head1.right, head2.right)
            elif head2.right:
                head1.right = head2.right
        head = root1
        if root1 is None:
            return root2
        dfs(root1, root2)
        return head
