package net.yutafujii.oauth2.authorization_server.repositories

import net.yutafujii.oauth2.authorization_server.domainModels.Client
import org.springframework.data.repository.CrudRepository

interface ClientRepository: CrudRepository<Client, Long> {
    fun findByClientId(clientId: String): Client?
}
