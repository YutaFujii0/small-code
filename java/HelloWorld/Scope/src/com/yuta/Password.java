package com.yuta;

public class Password {
    private static final int key = 3089;
    private final int encryptedPassword;

    public Password(int password) {
        this.encryptedPassword = encrypt(password);
        storePasswordPublic();
        storePasswordPrivate();
        storePasswordFinal();
        storePasswordPublicFinal();
    }

    public void storePasswordPublic() {
        // store it in your database for example.
        System.out.println("(public method) Password is stored as " + this.encryptedPassword);
    }

    private void storePasswordPrivate() {
        // store it in your database for example.
        System.out.println("(private method) Password is stored as " + this.encryptedPassword);
    }

    private final void storePasswordFinal() {
        // store it in your database for example.
        System.out.println("(private final method) Password is stored as " + this.encryptedPassword);
    }

    public final void storePasswordPublicFinal() {
        // store it in your database for example.
        System.out.println("(public final method) Password is stored as " + this.encryptedPassword);
    }

    public int encrypt(int password) {
        return this.key * password;
    }

    public boolean login(int password) {
        if (encrypt(password) == this.encryptedPassword) {
            System.out.println("Login");
            return true;
        } else {
            System.out.println("Password is incorrect.");
            return false;
        }
    }
}
