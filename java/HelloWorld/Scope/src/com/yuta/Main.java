package com.yuta;

public class Main {
    public static void main(String[] args) {
        SomeClass one = new SomeClass("one");
        SomeClass two = new SomeClass("two");
        SomeClass three = new SomeClass("three");

        // can't do this
        // three.instanceNumber = 4;

        // can't do this
        // Math m = new Math();

        // method scope 'public final'
        int pw = 52464;
        Password password = new Password(pw);
        password.login(1);
        password.login(-1);
        password.login(123);
        password.login(52464);

        Password extendedPassword = new SubPassword(pw);


        // static block
        SIBTest test = new SIBTest();
        System.out.println("SIBTest owner is " + test.owner);
    }
}
