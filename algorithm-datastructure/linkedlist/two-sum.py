from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# My Naive Solution
# sum = sentinel
# l1_cursor = l1
# l2_cursor = l2
# quota = 0
# if l1_cursor != None and l2_cursor != None
#  - calc quota & remaining of (two sum + previous quota)
#  - quota = refresh!
#  - sum.next = LinkNode(remaining)
#  - sum = sum.next
#  - l1_cursor = l1_cursor.next (and same as l2)
# if l1_cursor != None
#  - sum.next = l1_cursor(add quota every time)
# if l2_cursor != None
#  - sum.next = l2_cursor(add quota every time)
# return sentinel.next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        two_sum = ListNode()
        two_sum_cursor = two_sum
        l1_cursor = l1
        l2_cursor = l2
        quotient = 0
        while l1_cursor is not None or l2_cursor is not None:
            l1 = 0 if l1_cursor is None else l1_cursor.val
            l2 = 0 if l2_cursor is None else l2_cursor.val
            reminder = (l1 + l2 + quotient) % 10
            quotient = (l1 + l2 + quotient) // 10
            two_sum_cursor.next = ListNode(reminder)
            two_sum_cursor = two_sum_cursor.next
            l1_cursor = None if l1_cursor is None else l1_cursor.next
            l2_cursor = None if l2_cursor is None else l2_cursor.next
        if quotient != 0:
            two_sum_cursor.next = ListNode(quotient)
        return two_sum.next
