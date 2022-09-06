package com.yuta.ChainOfResponsibility;

public class Dialog extends AbstractWidget {

    private HelpHandler parent;
    public Dialog(HelpHandler parent) {
        this.parent = parent;
    }

    @Override
    public void handleHelp() {
        if (hasHelp()) {
            System.out.println("""
                    Help for dialog
                    1. Copy
                    2. Remove
                    3. Move
                    """);
        } else {
            parent.handleHelp();
        }
    }

    @Override
    public boolean hasHelp() {
        // what is it like in practical situation?
        return true;
    }

    public Button addButton() {
        return new Button(this);
    }

    public Text addText() {
        return new Text(this);
    }
}
