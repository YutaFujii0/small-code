/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */

// naive solution: 
// - first reverse linked list, 
// - remove nth node, and reverse again, 
// O(n) time complexity, and moreover, O(n) space complexity & fdata will be replaced
// Or, better one, scan total nodes, and then remove N - n th node
class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
//         count nodes
        ListNode cursor = head;
        int count = 1;
        while(cursor.next != null) {
            count++;
            cursor = cursor.next;
        }
        ListNode dummy = new ListNode(0, head);
        // System.out.println(count);
        ListNode prev = dummy;
        while(count != n) {
            prev = prev.next;
            count--;
        }
        prev.next = prev.next.next;
        return dummy.next;
    }
}