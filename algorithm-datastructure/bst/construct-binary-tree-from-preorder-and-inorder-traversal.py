from typing import List, Optional
from collections import deque
# preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
# preorder = [3,9,20,10,15,7], inorder = [10,9,3,15,20,7]
# preorder = [3,20,15,7,9], inorder = [15,20,9,7,3]

# first element of preorder array is the root node
# left of root node is left tree

# Psuedocode
# root =  node
# queue = [(inorder array, root)]
# while preorder array
# pop first node=head of preorder array
# - pop first of queue
# - find head in it
# - create node of head and connect it from parent
# - split it into left and right(and connection with parent) and push them into queue, only if split part is not empty
# return root.next

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        root = TreeNode(left=TreeNode())
        queue = deque([(inorder, root.left)])
        while preorder:
            headVal = preorder.pop(0)
            (nodes, newNode) = queue.popleft()
            head = TreeNode(val=headVal)
            newNode.val = headVal
            splitIndex = nodes.index(headVal)
            if not splitIndex == len(nodes) - 1:
                newNode.right = TreeNode()
                queue.appendleft((nodes[splitIndex+1:], newNode.right))
            if not splitIndex == 0:
                newNode.left = TreeNode()
                queue.appendleft((nodes[:splitIndex], newNode.left))
        return root.left
