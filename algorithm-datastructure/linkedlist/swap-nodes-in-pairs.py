from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# changed
# - pointer of head
# - pointer of latter node
# - pointer of former node
# - next

# two pointer approach
# set dummy head
# while head and head.next and head.next.next
#   tmp = head.next.next.next 3
#   tmpNode = head.next.next 2
#   head.next.next.next = head.next 1-2-1
#   head.next.next = tmp 1-3
#   head.next = tmpNode 2-1-3
#   head = head.next.next 1
# return dummy.next

class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        dummy.next = head
        head = dummy
        while head and head.next and head.next.next:
            tmp1 = head.next.next.next
            tmp2 = head.next.next
            head.next.next.next = head.next
            head.next.next = tmp1
            head.next = tmp2
            head = head.next.next
        return dummy.next
