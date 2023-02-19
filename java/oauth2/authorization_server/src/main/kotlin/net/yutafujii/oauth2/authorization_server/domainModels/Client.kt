package net.yutafujii.oauth2.authorization_server.domainModels

import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.Id

@Entity
class Client(
    var clientId: String,
    var clientSecret: String,
    @Id @GeneratedValue var id: Long? = null
)