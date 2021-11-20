from typing import Optional

# Set two pointer
# former = head.next
# latter = head
# return if
# while former is not null
#  if former.value == latter.value
#    (next iteration)
#  else
#    latter.next = former(remove duplicates)
#  former = former.next
# return head

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head == None:
            return head
        fast = head
        slow = head
        while fast != None and fast.next != None:
            fast = fast.next
            if fast.val != slow.val:
                slow.next = fast
                slow = slow.next
        slow.next = None
        return head
