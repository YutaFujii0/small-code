package com.yuta;

public class Main {
    public static void main(String[] args) {
        StringUtils utils = new StringUtils();
        StringBuilder sb = new StringBuilder();
        while (sb.length() < 10) {
            utils.addChar(sb, 'a');
        }
        System.out.println(sb);
    }
}

