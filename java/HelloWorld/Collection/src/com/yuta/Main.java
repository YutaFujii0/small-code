package com.yuta;

public class Main {
    public static void main(String[] args) {
        Theatre theatre = new Theatre("Waseda", 10, 12);
        theatre.listSeats();
        theatre.reserveSeat("A01");
        theatre.reserveSeat("A012");
    }
}
