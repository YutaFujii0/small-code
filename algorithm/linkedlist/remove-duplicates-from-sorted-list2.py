from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# two pointers, and using sentinel node
# one: hold current node and connect to next node when necessary
# two: move forward and decide which node to connect to
# ptr1 = None
# ptr2 = head
# see ptr2.next
# while ptr2 != none
# if values are the same before/after move
#  - move(see next node, not changing ptr2 value)
#  - if values are the same before/after move
#    - move(see next node, not changing ptr2 value)
#  - else
#    - set ptr2 to that node/None
# else
#  - connect ptr1 node to ptr2 node
#  - move ptr1 to next
#  - move ptr2 to next

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        ptr1 = None
        ptr2 = head
        new_head = None
        duplicate = False

        while ptr2 is not None and ptr2.next is not None:
            if ptr2.val == ptr2.next.val:
                duplicate = True
            else:
                if duplicate == False:
                    if new_head is None:
                        new_head = ptr2
                        ptr1 = new_head
                    else:
                        print(f"new connection! {ptr1.val} to {ptr2.val}")
                        ptr1.next = ptr2
                        ptr1 = ptr1.next
                duplicate = False
            print(f"prt2 value is {ptr2.val}")
            ptr2 = ptr2.next

        if ptr2.val == ptr1.val:
            ptr1.next = None
        else:
            ptr1.next = ptr2
        return new_head


a = [1,1,2,3,4,4,5]
head = ListNode(a[0])
cursor = head
i = 1
while i < len(a):
    cursor.next = ListNode(a[i])
    cursor = cursor.next
    i += 1

result = Solution().deleteDuplicates(head)
while result is not None:
    print(result.val)
    result = result.next
