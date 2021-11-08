# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# compare each first node and push one of less number
# till any of node become empty
# after that just connect remaining


class Solution:
    def mergeTwoLists(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if l1 == None and l2 == None:
            return None

        result_list = ListNode()
        left = l1
        right = l2
        head = result_list

        while left != None and right != None:
            if left.val > right.val:
                head.val = right.val
                right = right.next
            else:
                head.val = left.val
                left = left.next
            head.next = ListNode()
            head = head.next

        if left != None:
            head.val = left.val
            head.next = left.next
        if right != None:
            head.val = right.val
            head.next = right.next

        return result_list
