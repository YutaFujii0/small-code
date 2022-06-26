package com.yuta;

public class Main {
    public static void main(String[] arg) {
        // car
        Car porsche = new Car();
        Car holden = new Car();

        porsche.setModel("Carrera");
        System.out.println("Model is " + porsche.getModel());

        // bank account
        Account bobsAccount = new Account();
        bobsAccount.deposit(100);

        bobsAccount.withdraw(50);
        bobsAccount.withdraw(50);
        bobsAccount.withdraw(50);

        // animal
        Animal animal = new Animal("Animal", 1, 1, 5, 5);
        Dog dog = new Dog("Yorkie", 8, 20, 2, 4, 1, 20, "long silky");
        dog.eat();


        // composition
        Case theCase = new Case("Pilot", "Apple", new Dimensions(20, 20, 5));
        Monitor monitor = new Monitor("Violet", "SONY", new Resolution(2160, 1440));
        Motherboard motherboard = new Motherboard("M1", "Apple", 2, 0);
        PC pc = new PC(theCase, monitor, motherboard);
        pc.boot();
    }
}
