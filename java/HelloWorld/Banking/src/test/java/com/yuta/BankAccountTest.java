package com.yuta;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;

class BankAccountTest {

    private BankAccount account;

    @BeforeEach
    public void setup() {
        account = new BankAccount("Yuta", "fujii", 1000);
    }

    @Test
    void deposit() {
        int balance = account.deposit(200, true);
        assertEquals(1200, balance);
    }

    @ParameterizedTest
    @CsvSource(value = {"0:1000", "100:900", "500:500"}, delimiter = ':')
    void withdraw_shouldReturnTheBalance(int amount, int expected) {
        account.withdraw(amount, true);
        assertEquals(account.getBalance(), expected);
    }

    @Test
    void getBalance() {
        account.deposit(200, true);
        assertEquals(1200, account.getBalance());
    }


    // This will not be evaluated
    public void dummyTest() {
        fail("fail?");
    }
}