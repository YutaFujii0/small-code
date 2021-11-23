from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# Psuedocode
# stack = []
# sentinel node = ListNode
# cursor = sentinel node
# while head is not none
#  - store val in stack
# while stack is not empty
#  - cursor.next = pop val and create listnode
# return sentinel.next
# Complexity:
#  - time complexity: n * O(1) for push and n * O(1) for pop
#  - total O(n)
#  - space complexity: extra stack which at most n size, thus O(n)


# class Solution:
#     def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
#         stack = []
#         sentinel = ListNode()
#         cursor = sentinel
#         while head != None:
#             stack.append(head.val)
#             head = head.next
#         while len(stack) != 0:
#             cursor.next = ListNode(stack.pop())
#             cursor = cursor.next
#         return sentinel.next


# solution above is a bit less efficient in terms of space
# you don't need to use stack
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cursor = None
        while head != None:
            tmp = head.next
            head.next = cursor
            cursor = head
            head = tmp
        return cursor


# Solution2 - recursion
#  - sentinel = ListNode
#  - sentinel -> (reverseList of head.next) -> head
# Complexity:
#  - time complexity: each recursion, O(1) of head connection + O(1) cursor move
#  - total: O(n)
#  - space complexity: O(n) but much heavier than iteration
# class Solution:
#     def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
#         if head is None or head.next is None:
#             return head
#         tmp = head.next
#         new_head = self.reverseList(head.next)
#         head.next = None
#         cursor = new_head
#         tmp.next = head
#         return new_head
