package com.yuta;

public class Motherboard {
    private String model;
    private String manufacturer;
    private int ramSlots;
    private int cardSlots;

    public Motherboard(String model, String manufacturer, int ramSlots, int cardSlots) {
        this.model = model;
        this.manufacturer = manufacturer;
        this.ramSlots = ramSlots;
        this.cardSlots = cardSlots;
    }
}
