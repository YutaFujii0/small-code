package com.yuta.restDemo.account;

import com.yuta.restDemo.controller.AbstractController;
import org.hibernate.cfg.Configuration;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class AccountController extends AbstractController {

    Configuration cfg = new Configuration();

    @RequestMapping(path = "accounts", method = RequestMethod.GET)
    public String index() {
        String[] accounts = { "AN0001", "AN0001", "AN0001" };
        return accounts.toString();
    }

    @RequestMapping(path = "accounts/{id}", method = RequestMethod.GET)
    public String show(@PathVariable int id) {
        if (id == 0) {
            throw new RuntimeException("Here you see!");
        }
        return "AN0asdfa101" + id;
    }
}
