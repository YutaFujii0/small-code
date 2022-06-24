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

import java.util.AbstractMap;

class Solution {
    public ListNode mergeKLists(ListNode[] lists) {
        ListNode dummy = new ListNode();
        ListNode head = dummy;
        PriorityQueue<Map.Entry<Integer, Integer>> q = new PriorityQueue<>(Map.Entry.comparingByKey());

        int i = 0;
        for (ListNode listNode: lists) {
            // System.out.println(listNode.val);
            if (listNode != null) {
                q.add(new AbstractMap.SimpleEntry<>(listNode.val, i));
            }
            i++;
        }

        while (q.size() != 0) {
            Map.Entry<Integer, Integer> min = q.poll();
            head.next = new ListNode(min.getKey());
            head = head.next;

            // push next value to heap
            int index = min.getValue();
            lists[index] = lists[index].next;
            if (lists[index] != null) {
                q.add(new AbstractMap.SimpleEntry<>(lists[index].val, index));
            }
        }
        return dummy.next;
    }
}
