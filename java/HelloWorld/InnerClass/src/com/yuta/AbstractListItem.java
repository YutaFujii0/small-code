package com.yuta;

public abstract class AbstractListItem {
    private AbstractListItem prev;
    private AbstractListItem next;
    private int value;

    public AbstractListItem(int value) {
        this.value = value;
    }


    public AbstractListItem getPrev() {
        return prev;
    }

    public void setPrev(AbstractListItem prev) {
        this.prev = prev;
    }

    public AbstractListItem getNext() {
        return next;
    }

    public void setNext(AbstractListItem next) {
        this.next = next;
    }

    public int getValue() {
        return value;
    }

    public void setValue(int value) {
        this.value = value;
    }

    public int compareTo(AbstractListItem another) {
        if (this.value == another.getValue()) {
            return 0;
        } else if (this.value > another.getValue()) {
            return 1;
        } else {
            return -1;
        }
    }
}
