# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

# Use slow and fast cursor method
# when collision occurs, mathematically below equation meets:
#  a = b - c
#  where
#  a: distance(steps) from head to loop-starting node
#  b: distance of loop
#  c: distance from loop-starting node to collision node
# Thus,
#  distance from head to loop-starting node = distance from collision node to loop-starting node
# Time complexity: O(2n), Space complexity: O(1)
class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        slow = head
        fast = head
        while fast != None and fast.next != None:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                cur = head
                while cur != slow:
                    cur = cur.next
                    slow = slow.next
                return cur
        return None
