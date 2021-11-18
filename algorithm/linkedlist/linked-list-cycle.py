# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

# pos is only used in explanation, not given as arguments or needed to return
# naively thought, every time check if next has already traversed.
# store node
# move to next
# check if node is already stored
# if it is, return true
# else move next
# iterate above
# return false
# time complexity: sigma(k * (k-1)) = O(n^2)
# space complexity: O(n) for store

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
