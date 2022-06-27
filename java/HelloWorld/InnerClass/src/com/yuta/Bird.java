package com.yuta;

public abstract class Bird extends Animal {
    public Bird(String name) {
        super(name);
    }

    @java.lang.Override
    public void eat() {
        System.out.println(getName() + " is pecking");
    }

    @java.lang.Override
    public void breathe() {
        System.out.println("Breathe in, breathe out, repeat");
    }

    public abstract void fly();
}
