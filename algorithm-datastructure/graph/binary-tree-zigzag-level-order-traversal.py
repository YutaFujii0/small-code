from typing import Optional, List
from collections import deque
# BFS with traversal order
# stack(LIFO)
# Psuedocode
# return array = []
# queue = [stack(root)]
# right = false
# while queue is not empty
# - stack = []
# - nums = []
# - for node in queue.pop[0]
#  - if node explored, continue
#  - mark node as explored
#  - push stack left/right node
# - push stack to queue
# - push nums to return array
# Complexity:
# - time complexity: O(n + m) where n ndoes, m edges
# - space complexity: O(n) for explored, extra stack,queue

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        values = []
        queue = deque([deque([root])])
        explored = set()
        switch = False
        while queue:
            stack = deque()
            nums = []
            nodes = queue.popleft()
            if len(nodes) == 0:
                break
            while nodes:
                node = nodes.pop()
                if node in explored or node is None:
                    continue
                explored.add(node)
                nums.append(node.val)
                if switch:
                    stack.append(node.right)
                    stack.append(node.left)
                else:
                    stack.append(node.left)
                    stack.append(node.right)
            queue.append(stack)
            if nums: values.append(nums)
            switch = not switch
        return values
