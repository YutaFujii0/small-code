package com.yuta;

import java.util.Collection;
import java.util.LinkedHashSet;

public class Theatre {
    private final String name;
    private Collection<Seat> seats = new LinkedHashSet<>();
//    private List<Seat> seats = new ArrayList<>();

    public Theatre(String name, int numRows, int seatsPerRow) {
        this.name = name;
        int lastRow = 'A' + (numRows - 1);
        for (char row = 'A'; row <= lastRow; row++) {
            for (int i = 1; i <= seatsPerRow; i++) {
                Seat seat = new Seat(row + String.format("%02d", i));
                seats.add(seat);
            }
        }
    }

    public String getName() {
        return name;
    }

    public boolean reserveSeat(String seatNumber) {
        Seat requestSeat = null;
        for (Seat seat: seats) {
            if (seat.seatNumber == seatNumber) {
                requestSeat = seat;
            }
        }
        if (requestSeat == null) {
            System.out.println("There is no seat" + seatNumber);
            return false;
        } else {
            return requestSeat.reserve();
        }
    }

    public void listSeats() {
        for (Seat seat: seats) {
            System.out.println(seat.getSeatNumber());
        }
    }

    private class Seat implements Comparable<Seat> {
        private final String seatNumber;
        private boolean reserved = false;

        public Seat(String seatNumber) {
            this.seatNumber = seatNumber;
        }

        @Override
        public int compareTo(Seat seat) {
            return this.seatNumber.compareToIgnoreCase(seat.getSeatNumber());
        }

        public String getSeatNumber() {
            return seatNumber;
        }

        public boolean reserve() {
            if (!this.reserved) {
                this.reserved = true;
                System.out.println("Seat " + seatNumber + "reserved");
                return true;
            }
            System.out.println("Seat is already taken");
            return false;
        }
    }
}
