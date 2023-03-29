package net.yutafujii.oauth2.client.controllers

import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RequestMapping("/actions")
@RestController
class ActionController {

    @GetMapping("")
    fun getActions(): ResponseEntity<String> {

        return ResponseEntity("Hi", HttpStatus.OK);
    }
}