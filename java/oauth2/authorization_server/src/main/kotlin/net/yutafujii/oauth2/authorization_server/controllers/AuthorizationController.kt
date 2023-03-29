package net.yutafujii.oauth2.authorization_server.controllers

import net.yutafujii.oauth2.authorization_server.domainModels.ResponseType
import net.yutafujii.oauth2.authorization_server.repositories.ClientRepository
import org.apache.coyote.http11.Constants.a
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController

@RestController
class AuthorizationController(
    var clientRepository: ClientRepository
) {

    /**
     * RFC6749 4.1.1 Authorization Request
     */
    @GetMapping("/authorize")
    fun authorize(
        @RequestParam("response_type") responseType: ResponseType,
        @RequestParam("client_id") clientId: String,
        @RequestParam("redirect_uri", required = false) redirectUri: String?,
        @RequestParam(required = false) scope: String?,
        @RequestParam(required = false) state: String?,
    ): ResponseEntity<String> {
        // no need to authenticate client
        var client = clientRepository.findByClientId(clientId)


        // validate redirectUri in allowedlist


        return ResponseEntity(client?.clientId, HttpStatus.OK);
    }
}