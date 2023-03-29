package net.yutafujii.oauth2.authorization_server

import net.yutafujii.oauth2.authorization_server.domainModels.Client
import net.yutafujii.oauth2.authorization_server.repositories.ClientRepository
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class AuthorizationServerApplication

fun main(args: Array<String>) {
	var context = runApplication<AuthorizationServerApplication>(*args)

	var clientRepository = context.getBean(ClientRepository::class.java)
	var clients = listOf(
		Client("1234", "1234"),
		Client("abcd", "efgh")
	)
	clients.forEach{ client -> clientRepository.save(client)}
}

