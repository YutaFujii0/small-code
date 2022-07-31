package com.yuta.restDemo.controller;

import org.springframework.stereotype.Controller;

@Controller
public abstract class AbstractController {
    public abstract String index();
    public abstract String show(int id);
//    abstract Object create(Object params);
//    abstract Object edit(int id);
//    abstract Object update(Object params);
//    abstract Object delete(Object params);
}
