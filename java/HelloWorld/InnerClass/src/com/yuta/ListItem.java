package com.yuta;

public class ListItem extends AbstractListItem {
    public ListItem(Object value) {
        super(value);
    }

    @java.lang.Override
    AbstractListItem prev() {
        return this.prev;
    }

    @java.lang.Override
    AbstractListItem next() {
        return this.next;
    }

    @java.lang.Override
    AbstractListItem setPrev(AbstractListItem item) {
        this.prev = item;
        return this.prev;
    }

    @java.lang.Override
    AbstractListItem setNext(AbstractListItem item) {
        this.next = item;
        return this.next;
    }

    @java.lang.Override
    int compareTo(AbstractListItem another) {
        if (another != null) {
            return (getValue().toString()).compareTo(another.getValue().toString());
        } else {
            return -1;
        }
    }
}
