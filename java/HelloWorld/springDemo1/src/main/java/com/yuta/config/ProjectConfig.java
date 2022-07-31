package com.yuta.config;

import com.yuta.beans.Vehicle;
import org.springframework.beans.factory.annotation.Configurable;
import org.springframework.context.annotation.Bean;

@Configurable
public class ProjectConfig {

    @Bean
    Vehicle vehicle() {
        var veh = new Vehicle();
        veh.setName("Audi 8");
        return veh;
    }

    @Bean
    public String hello() {
        return "Hello World";
    }

    @Bean
    Integer number() {
        return 16;
    }
}
