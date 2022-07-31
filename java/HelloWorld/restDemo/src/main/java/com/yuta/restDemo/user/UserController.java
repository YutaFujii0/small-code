package com.yuta.restDemo.user;

import com.yuta.restDemo.interfaces.LogAspect;
import org.apache.commons.logging.Log;
import org.apache.commons.logging.LogFactory;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class UserController {

    private Log log = LogFactory.getLog(UserController.class);

    @GetMapping(path="hi")
    @LogAspect
    String hi() {
        log.info("-----------------   hi   aaa  ------------------");
        return "his";
    }

}
