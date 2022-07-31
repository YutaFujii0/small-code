package com.yuta.restDemo.controller;


import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.MessageSource;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.context.request.WebRequest;
import org.springframework.web.servlet.mvc.method.annotation.ResponseEntityExceptionHandler;

import java.util.Date;
import java.util.Locale;

@ControllerAdvice
@RestController
public final class GlobalExceptionHandler extends ResponseEntityExceptionHandler {

    @Autowired
    private MessageSource messageSource;

    @ExceptionHandler(Exception.class)
    public final ResponseEntity<Object> exceptionHandler(Exception e, WebRequest req) {
        String msg = messageSource.getMessage("greeting", null, Locale.JAPANESE);
        System.out.println(String.format("-------- %s ----------", msg));
        ExceptionResponse response = new ExceptionResponse(new Date(),
                e.getMessage(),
                req.getDescription(false));

        return new ResponseEntity(response, HttpStatus.INTERNAL_SERVER_ERROR);
    }
}
