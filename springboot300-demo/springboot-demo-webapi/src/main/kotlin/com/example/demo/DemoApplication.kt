package com.example.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

import com.fasterxml.jackson.module.kotlin.*
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.slf4j.MDC
import org.springframework.beans.factory.annotation.Value
import org.springframework.boot.context.event.ApplicationReadyEvent
import org.springframework.context.event.EventListener

@SpringBootApplication
public class DemoApplication {

    private val logger: Logger = LoggerFactory.getLogger(DemoApplication::class.java)

    @Value("\${server.port}")
    private val port: String? = null

    @EventListener
    fun onStartup(event: ApplicationReadyEvent?) {
        logger.info(String.format("Application running http://localhost:%s", port))

        MDC.put("logFileName", "demo-webapi.log")
        try {
            logger.error("test error")
        }
        finally {
            MDC.remove("logFileName")
        }
    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
