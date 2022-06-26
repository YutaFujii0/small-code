package com.yuta;

public class Case {
    private String model;
    private String manufacturer;
    private Dimensions dimensions;

    public Case(String model, String manufacturer, Dimensions dimensions) {
        this.model = model;
        this.manufacturer = manufacturer;
        this.dimensions = dimensions;
    }

    public void pressStartButton() {
        System.out.println("Start button pressed");
    }
}
