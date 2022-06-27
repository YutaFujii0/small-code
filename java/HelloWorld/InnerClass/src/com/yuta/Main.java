package com.yuta;

public class Main {
    public static void main(String[] args) {
        MyLinkedList linkedList = new MyLinkedList();
        ListItem item1 = new ListItem(10);
        ListItem item2 = new ListItem(5);
        ListItem item3 = new ListItem(7);
        ListItem item4 = new ListItem(1);
        ListItem item5 = new ListItem(2);
        ListItem item6 = new ListItem(3);
        ListItem item7 = new ListItem(4);
        ListItem item8 = new ListItem(8);
        ListItem item9 = new ListItem(1);
        System.out.println(linkedList.add(item1));
        System.out.println(linkedList.add(item2));
        System.out.println(linkedList.add(item3));
        System.out.println(linkedList.add(item4));
        System.out.println(linkedList.add(item5));
        System.out.println(linkedList.add(item6));
        System.out.println(linkedList.add(item7));
        System.out.println(linkedList.add(item8));
        System.out.println(linkedList.add(item9));
        linkedList.traversePrev();
        linkedList.traverseNext();
    }
}
