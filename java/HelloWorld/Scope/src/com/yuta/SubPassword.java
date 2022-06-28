package com.yuta;

public class SubPassword extends Password {
    public SubPassword(int password) {
        super(password);
    }

    @Override
    public void storePasswordPublic() {
        // store it in your database for example.
        System.out.println("(public method) Password is stored as " + 1);
    }

    // @Override // You cannot override
    private void storePasswordPrivate() {
        // store it in your database for example.
        System.out.println("(private method) Password is stored as " + 1);
    }

    // @Override // You cannot override
    private final void storePasswordFinal() {
        // store it in your database for example.
        System.out.println("(private final method) Password is stored as " + 1);
    }

    // @Override // You cannot override
//    public void storePasswordFinal() {
//        // store it in your database for example.
//        System.out.println("(private final method) Password is stored as " + 1);
//    }
}
