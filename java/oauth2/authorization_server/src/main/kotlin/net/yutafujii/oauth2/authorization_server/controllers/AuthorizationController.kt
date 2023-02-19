package net.yutafujii.oauth2.authorization_server.controllers

import net.yutafujii.oauth2.authorization_server.domainModels.ResponseType
import net.yutafujii.oauth2.authorization_server.repositories.ClientRepository
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController

@RestController
class AuthorizationController(
    var clientRepository: ClientRepository
) {
    @GetMapping("/authorize")
    fun authorize(
        @RequestParam("response_type") responseType: ResponseType,
        @RequestParam("client_id") clientId: String,
        @RequestParam("redirect_uri") redirectUri: String,
    ): ResponseEntity<String> {

        // validate redirectUri in allowedlist

        var client = clientRepository.findByClientId(clientId)
        return ResponseEntity(client?.clientId, HttpStatus.OK);
    }
}