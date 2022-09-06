package com.yuta.ChainOfResponsibility;

public class Text extends AbstractWidget {
    private HelpHandler parent;

    public Text(HelpHandler parent) {
        this.parent = parent;
    }

    @Override
    public void handleHelp() {
        if (hasHelp()) {
            System.out.println("Text help");
            // never come here...
        } else {
            parent.handleHelp();
        }
    }

    @Override
    public boolean hasHelp() {
        return false;
    }
}
