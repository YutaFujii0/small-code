from typing import Optional
# problem: can't know how long the list is, thus nth from the end seems
# unknown at the beginning

# naive solutinon: reverse list
# Complexity:
# - time complexity: O(2n) for 2 times reverse
# - space complexity: O(1) for additional head

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        revHead = None
        while head:
            tmp = head
            head = head.next
            tmp.next = revHead
            revHead = tmp

        i = 1

        while revHead:
            if i == n:
                revHead = revHead.next
                i += 1
                continue
            tmp = revHead
            revHead = revHead.next
            tmp.next = head
            head = tmp
            i += 1

        return head
