package com.yuta;

public class MyLinkedList {
    private ListItem root;

    public MyLinkedList() {
        this.root = null;
    }

    public ListItem getRoot() {
        return root;
    }

    public void traverseNext() {
        ListItem cur = root;
        while (cur != null) {
            System.out.println(cur.getValue());
            cur = cur.getNext();
        }
    }

    public void traversePrev() {
        ListItem cur = root;
        while (cur != null) {
            System.out.println(cur.getValue());
            cur = cur.getPrev();
        }
    }

    public boolean add(ListItem listItem) {
        if (root == null) {
            this.root = listItem;
            return true;
        } else {
            ListItem cur = root;
            while (cur != null) {

                switch (cur.compareTo(listItem)) {
                    case 0:
                        return false;
                    case 1:
                        if (cur.getPrev() == null) {
                            cur.setPrev(listItem);
                            listItem.setNext(cur);
                            return true;
                        } else if (cur.getPrev().compareTo(listItem) == -1) {
                            ListItem prev = cur.getPrev();
                            cur.setPrev(listItem);
                            listItem.setNext(cur);
                            listItem.setPrev(prev);
                            prev.setNext(listItem);
                            return true;
                        } else {
                            cur = cur.getPrev();
                            break;
                        }

                    case -1:
                        if (cur.getNext() == null) {
                            cur.setNext(listItem);
                            listItem.setPrev(cur);
                            return true;
                        } else if (cur.getNext().compareTo(listItem) == 1) {
                            ListItem next = cur.getNext();
                            cur.setNext(listItem);
                            listItem.setPrev(cur);
                            listItem.setPrev(next);
                            next.setPrev(listItem);
                            return true;
                        } else {
                            cur = cur.getNext();
                            break;
                        }
                }
            }
            return false;
        }
    }
}
