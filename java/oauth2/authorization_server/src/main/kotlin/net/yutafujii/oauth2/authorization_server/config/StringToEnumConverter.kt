package net.yutafujii.oauth2.authorization_server.config

import net.yutafujii.oauth2.authorization_server.domainModels.ResponseType
import org.springframework.core.convert.converter.Converter

class StringToEnumConverter : Converter<String, ResponseType> {
    override fun convert(source: String): ResponseType? {
        return ResponseType.valueOf(source.uppercase())
    }
}