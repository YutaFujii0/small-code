package com.yuta;

public class BankAccount {
    private String firstName;
    private String lastName;
    private int balance;

    public BankAccount(String firstName, String lastName, int balance) {
        this.firstName = firstName;
        this.lastName = lastName;
        this.balance = balance;
    }

    public int deposit(int amount, boolean branch) {
        balance += amount;
        return balance;
    }

    public int withdraw(int amount, boolean branch) {
        balance -= amount;
        return balance;
    }

    public int getBalance() {
        return balance;
    }
}
