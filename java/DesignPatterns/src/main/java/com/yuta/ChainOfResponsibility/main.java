package com.yuta.ChainOfResponsibility;

public class main {

    public static void main(String[] args) {
        Application app = new Application();
        Dialog dialog = new Dialog(app);
        Button button = dialog.addButton();
        Text text1 = dialog.addText();
        button.handleHelp();
        text1.handleHelp();
    }
}
