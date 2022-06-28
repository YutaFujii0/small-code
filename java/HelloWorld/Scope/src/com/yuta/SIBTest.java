package com.yuta;

public class SIBTest {
    public static final String owner;

    static {
        System.out.println("First static block called");
    }

    static {
        System.out.println("Second static block called");
        owner = "me";
    }

    static {
        System.out.println("Third static block called");
    }

    public SIBTest() {
        System.out.println("Instantiated");
    }
}
