package com.example.demo

import ch.qos.logback.classic.ClassicConstants
import ch.qos.logback.classic.LoggerContext
import ch.qos.logback.classic.encoder.PatternLayoutEncoder
import ch.qos.logback.classic.sift.MDCBasedDiscriminator
import ch.qos.logback.classic.sift.SiftingAppender
import ch.qos.logback.classic.spi.ILoggingEvent
import ch.qos.logback.core.FileAppender
import ch.qos.logback.classic.BasicConfigurator
import ch.qos.logback.classic.AsyncAppender
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.slf4j.Marker
import org.slf4j.MarkerFactory

// https://stackoverflow.com/questions/44915798/slf4j-logger-using-logback-logs-multiple-times-depending-on-the-number-of-thread
// https://logback.qos.ch/manual/loggingSeparation.html
// https://www.codetd.com/en/article/14064661

// logback.ContextSelector


//object LogUtil {
//    val marker: Marker
//        get() {
//            val marker: Marker = MarkerFactory.getMarker("Finalize")
//            marker.add(ClassicConstants.FINALIZE_SESSION_MARKER)
//            return marker
//        }
//
//    fun addSiftingAppender(log: Logger) {
//        val lc: LoggerContext = LoggerFactory.getILoggerFactory() as LoggerContext
//        val logger: Logger = lc.getLogger(log.name)
//
//        val sa = SiftingAppender()
//        sa.name = "SIFT"
//        sa.context = lc
//
//        val discriminator = MDCBasedDiscriminator()
//        discriminator.key = "logFileName"
//        discriminator.defaultValue = "batch0"
//        discriminator.start()
//
//        sa.discriminator = discriminator
//        sa.setAppenderFactory { context, discriminatingValue ->
//            val appender = FileAppender<ILoggingEvent>()
//            appender.name = "BATCH-EXECUTION-$discriminatingValue"
//            appender.context = context
//            appender.file = "$discriminatingValue.log"
//            appender.isAppend = true
//            if (appender.isStarted) {
//                appender.stop()
//            }
//            val pl = PatternLayoutEncoder()
//            pl.context = context
//            pl.pattern = "%d{HH:mm:ss.SSS} [%thread] %-5level %logger{36} - %msg%n"
//            pl.setImmediateFlush(true)
//            if (pl.isStarted) {
//                pl.stop()
//            }
//            pl.start()
//            appender.setEncoder(pl)
//            appender.start()
//            appender
//        }
//        if (sa.isStarted) {
//            sa.stop()
//        }
//        sa.start()
//        logger.addAppender(sa)
//    }
//}
