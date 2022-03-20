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


class Solution2:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        head = pointer = ListNode()
        h = []
        for i, l in enumerate(lists):
            if l is None: continue
            heappush(h, (l.val, i))
            lists[i] = lists[i].next

        while h:
            val, i = heappop(h)
            pointer.next = ListNode(val)
            pointer = pointer.next
            if lists[i]:
                heappush(h, (lists[i].val, i))
                lists[i] = lists[i].next

        return head.next
