from typing import Optional, List
# heap
# lots of "get min" -> heap may fit
# initial state
# push each head of k linked lists into heap
# while heap

# idea push everything into array and execute merge sort
# O(n) + O(nlogn)

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        arr = []
        for l in lists:
            while l:
                arr.append(l.val)
                l = l.next
        arr.sort()
        head = ListNode()
        tmp = head
        for num in arr:
            tmp.next = ListNode(num)
            tmp = tmp.next
        return head.next
