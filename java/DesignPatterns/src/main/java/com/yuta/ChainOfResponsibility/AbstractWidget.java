package com.yuta.ChainOfResponsibility;

public abstract class AbstractWidget implements HelpHandler {
    private HelpHandler parent;

    public abstract void handleHelp();

    public abstract boolean hasHelp();
}
