from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


# My First Solution
# create special node
# get pointer to next node
# set next node of current traversal to special node
# move to next
# get pointer to next node
# set next node of current traversal to special node
# if next node is special node, it means cycle exists
class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head == None:
            return False
        special_node = ListNode(0)
        next_node = head.next
        head.next = special_node
        while next_node != None and next_node != special_node:
            tmp = next_node.next
            next_node.next = special_node
            next_node = tmp
        return next_node == special_node


# Best Practice Solution
# Floyd’s Cycle Detection Algorithm.
# ... This algorithm is applied when you want to determine
# where a data structure(graph, linkedlist, etc) has cycle
# cf https://iq.opengenus.org/fast-and-slow-pointer-technique/
# Set two pointers(slow, fast)
# slow will be advanced one step each,
# whereas fast moves more than one step
# if both pointer points the same address, that means a cycle exists
# Time complexity: O(n), Space complexity: O(1)
class Solution2:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        slow = head
        fast = head
        while fast != None and fast.next != None:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                return True
        return False
