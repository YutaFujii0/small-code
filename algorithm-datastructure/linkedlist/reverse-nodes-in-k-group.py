from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# reverse(head)
# - if head.next == None: return head
# - reverse(head.next) -> head
# - return head

# split k nodes -> break and return if list ends
# tmp = nextNode
# reverse those nodes
# connect them to tmp

# for i in k:
#   head = head.next

# Complexity:
#  - time complexity: O(n)
#  - space complexity: O(1)... or O(n) for recursion

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        def reverse(head):
            if head.next is None:
                return head
            reverse(head.next).next = head
            return head
        
        dummy = ListNode()
        dummy.next = head
        head = dummy

        while head:
            fast = head
            for _ in range(k):
                if fast.next is None:
                    return dummy.next
                fast = fast.next
            tmp = fast.next
            tail = head.next
            fast.next = None
            reverse(head.next).next = tmp
            head.next = fast
            head = tail

        return dummy.next
