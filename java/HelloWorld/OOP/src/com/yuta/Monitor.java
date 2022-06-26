package com.yuta;

public class Monitor {
    private String name;
    private String manufacturer;
    private Resolution nativeResolution;

    public Monitor(String name, String manufacturer, Resolution nativeResolution) {
        this.name = name;
        this.manufacturer = manufacturer;
        this.nativeResolution = nativeResolution;
    }

    public void drawBootMsg() {
        System.out.println("Your PC is booting...");
    }
}
