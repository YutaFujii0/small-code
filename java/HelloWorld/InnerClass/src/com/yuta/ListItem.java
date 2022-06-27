package com.yuta;

public class ListItem extends AbstractListItem {
    public ListItem(int value) {
        super(value);
    }

    @java.lang.Override
    public ListItem getPrev() {
        return (ListItem) super.getPrev();
    }

    @java.lang.Override
    public ListItem getNext() {
        return (ListItem) super.getNext();
    }
}
