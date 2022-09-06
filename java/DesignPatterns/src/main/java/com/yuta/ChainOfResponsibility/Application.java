package com.yuta.ChainOfResponsibility;

public class Application implements HelpHandler {

    @Override
    public void handleHelp() {
        System.out.println("This is the last resort help.");
    }

    @Override
    public boolean hasHelp() {
        return true;
    }
}
