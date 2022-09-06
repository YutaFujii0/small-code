package com.yuta.ChainOfResponsibility;

public class Button extends AbstractWidget {

    private HelpHandler parent;

    public Button(HelpHandler parent) {
        this.parent = parent;
    }

    @Override
    public void handleHelp() {
        if (hasHelp()) {
            String helpMenu = """
                    Button help menu
                    1. Copy
                    2. Cut
                    3. Remove
                    """;
            System.out.println(helpMenu);
        } else {
            parent.handleHelp();
        }
    }

    @Override
    public boolean hasHelp() {
        return true;
    }
}
