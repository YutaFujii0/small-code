package com.yuta;

public class Account {
    private String number;
    private int balance;
    private String customerName;
    private String customerEmailAddress;
    private String customerPhoneNumber;

    public Account() {
        this("01324", 100, "Alice",
                "example@example.com", "04234098020");
        System.out.println("Empty constructor called");
    }

    public Account(String number, int balance, String customerName,
                   String customerEmailAddress, String customerPhoneNumber) {
//        this.number = number;
//        this.balance = balance;
//        this.customerName = customerName;
//        this.customerEmailAddress = customerEmailAddress;
//        this.customerPhoneNumber = customerPhoneNumber;
        setNumber(number);
        setBalance(balance);
        setCustomerName(customerName);
        setCustomerEmailAddress(customerEmailAddress);
        setCustomerPhoneNumber(customerPhoneNumber);
    }

    public Account(String customerName, String customerEmailAddress, String customerPhoneNumber) {
        this("99999", 100, customerName,
                customerEmailAddress, customerPhoneNumber);
    }

    public void deposit(int depositAmount) {
        this.balance += depositAmount;
        System.out.println("Deposit of " + depositAmount + ". Now amount is" + balance);
    }

    public void withdraw(int withdrawAmount) {
        if(this.balance - withdrawAmount < 0) {
            System.out.println("Only" + balance + " available. Withdrawal not processed");
        } else {
            balance -= withdrawAmount;
            System.out.println("Withdrawal of " + withdrawAmount + ". Now amount is" + balance);
        }
    }

    public String getNumber() {
        return number;
    }

    public void setNumber(String number) {
        this.number = number;
    }

    public int getBalance() {
        return balance;
    }

    public void setBalance(int balance) {
        this.balance = balance;
    }

    public String getCustomerName() {
        return customerName;
    }

    public void setCustomerName(String customerName) {
        this.customerName = customerName;
    }

    public String getCustomerEmailAddress() {
        return customerEmailAddress;
    }

    public void setCustomerEmailAddress(String customerEmailAddress) {
        this.customerEmailAddress = customerEmailAddress;
    }

    public String getCustomerPhoneNumber() {
        return customerPhoneNumber;
    }

    public void setCustomerPhoneNumber(String customerPhoneNumber) {
        this.customerPhoneNumber = customerPhoneNumber;
    }
}
