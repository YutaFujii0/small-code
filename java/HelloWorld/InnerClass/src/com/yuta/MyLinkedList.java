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
        AbstractListItem cur = root;
        while (cur != null) {
            System.out.println(cur.getValue());
            cur = cur.next();
        }
    }

    public void traversePrev() {
        AbstractListItem cur = root;
        while (cur != null) {
            System.out.println(cur.getValue());
            cur = cur.prev();
        }
    }

    public boolean add(ListItem item) {
        if (this.root == null) {
            this.root = item;
            return true;
        }

        AbstractListItem cur = this.root;
        while (cur != null) {
            int comp = cur.compareTo(item);
            if (comp == 0) {
                // item already exists.
                return false;
            } else if (comp < 0) {
                // item is greater than cur
                if (cur.next() == null) {
                    cur.setNext(item).setPrev(cur);
                    return true;
                } else {
                    cur = cur.next();
                }
            } else {
                // cur.prev -> item -> cur  is the right order
                // take care of the case where cur is root
                if (cur.prev() == null) {
                    this.root = item;
                    item.setNext(cur).setPrev(cur);
                } else {
                    cur.prev().setNext(item).setPrev(cur.prev());
                    item.setNext(cur).setPrev(item);
                }
                return true;
            }
        }
        return false;
    }
}
