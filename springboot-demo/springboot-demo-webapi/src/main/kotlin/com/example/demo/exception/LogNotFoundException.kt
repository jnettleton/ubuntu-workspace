package com.example.demo.exception

import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.ControllerAdvice
import org.springframework.web.bind.annotation.ExceptionHandler
import javax.servlet.http.HttpServletRequest

class LogNotFoundException : Exception()

@ControllerAdvice
class LogErrorHandler {
    @ExceptionHandler(LogNotFoundException::class)
    fun handleLogNotFoundException(
        servletRequest: HttpServletRequest,
        exception: Exception
    ): ResponseEntity<String> {
        return ResponseEntity("Log not found", HttpStatus.NOT_FOUND)
    }
}
