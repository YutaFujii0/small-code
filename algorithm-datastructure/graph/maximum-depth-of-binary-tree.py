from typing import Optional

# BFS and determining layers
# return the deepest layer

# explored = {}
# queue = [head]
# layer = 0
# while queue and queue.first
#  - for item in queue.first
#   - head = queue.pop
#   - if head is explored, or none, continue next
#   - mark head as explored
#   - next-layer-items.push head.left and head.right
#  - queue push next layer items
#  - layer ++
# return layer
# Complexity
# - time complexity: O(1) * n
# - space complexity: O(n) for extra hashset, queue

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        explored = set()
        queue = [[root]]
        layer = 0
        while queue and queue[0]:
            next_layer = []
            for node in queue.pop(0):
                if node is None or node in explored:
                    continue
                explored.add(node)
                next_layer.append(node.left)
                next_layer.append(node.right)
            queue.append(next_layer)
            layer += 1
        return layer - 1
