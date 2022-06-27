package com.yuta;

public abstract class AbstractListItem {

    // prefer protected rather than private:
    // enable subclasses in the same package to access
    // cf) https://docs.oracle.com/javase/tutorial/java/javaOO/accesscontrol.html
    protected AbstractListItem prev = null;
    protected AbstractListItem next = null;
    protected Object value;

    public AbstractListItem(Object value) {
        this.value = value;
    }

    abstract AbstractListItem prev();
    abstract AbstractListItem next();
    abstract AbstractListItem setPrev(AbstractListItem prev);
    abstract AbstractListItem setNext(AbstractListItem next);
    abstract int compareTo(AbstractListItem another);

    public Object getValue() {
        return value;
    }

    public void setValue(Object value) {
        this.value = value;
    }
}
