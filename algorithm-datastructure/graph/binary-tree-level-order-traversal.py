from typing import Optional, List
# Layer determination using BFS
# Psuedocode
# queue = [[head]]
# explored = set()
# while queue and queue[0]
# - next_items
# - layer
# - for node in queue.pop(0)
#  - if node in explored:
#    continue
#  - mark node as explored
#  - push left and right node to next_items
#  - push val to layer
# - push next_items to queue
# - push layer to layers
# Complexity
# - time complexity: O(n + m)
# - space complexity: O(n)

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        queue = [[root]]
        explored = set()
        layers = []
        while queue and queue[0]:
            next_nodes = []
            layer = []
            for node in queue.pop(0):
                if node in explored:
                    continue
                explored.add(node)
                if node.left: next_nodes.append(node.left)
                if node.right: next_nodes.append(node.right)
                layer.append(node.val)
            layers.append(layer)
            if next_nodes:
                queue.append(next_nodes)

        return layers
