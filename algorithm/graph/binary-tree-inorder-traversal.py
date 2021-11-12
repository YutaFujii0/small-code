# binary tree 1d array means
# [1,2,3,4,5,6,7,8]
# 1 -> 2,3
# 2 -> 4,5
# 3 -> 6,7
# 4 -> 8

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


# set return array
# see left
# - if left is not empty
#   - <same procedure to left node>
# - if left is empty
#   - see right
#     - if right is not empty
#       - <same procedure to right node>
#     - if right is empty
#       - do nothing
# - push val to return array

class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        self.return_array = []
        self.traversal(root)
        return self.return_array
    
    def traversal(self, node: Optional[TreeNode]):
        if node == None:
            return
        if node.left != None:
            self.traversal(node.left)
        self.return_array.append(node.val)
        if node.right != None:
            self.traversal(node.right)
        


# space complexity: worst senario O(n), on average O(log n)
# this is because every recursion traverse method is push on stack
#  meaning using extra space
