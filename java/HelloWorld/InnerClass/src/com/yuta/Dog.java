package com.yuta;

public class Dog extends Animal {
    public Dog(String name) {
        super(name);
    }

    @java.lang.Override
    public void eat() {
        System.out.println(getName() + " is eating");
    }

    @java.lang.Override
    public void breathe() {
        System.out.println("Breathe in, breathe out, repeat");
    }
}
