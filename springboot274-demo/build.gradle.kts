import java.nio.charset.Charset
import io.spring.gradle.dependencymanagement.dsl.DependencyManagementExtension

val kotlinVersion: String by project
val kotlinLoggingVersion: String by project
val logbackVersion: String by project

plugins {
	id("org.springframework.boot") version "2.7.4" apply false
	id("io.spring.dependency-management") version "1.1.0"
	kotlin("jvm") version "1.6.21"
	kotlin("plugin.spring") version "1.6.21"
}

buildscript {
	val kotlinVersion: String by project
	val springDependencyManagementVersion: String by project

	repositories {
		mavenLocal()
		mavenCentral()
		maven { url = uri("https://plugins.gradle.org/m2/") }
	}

	dependencies {
		classpath("org.jfrog.buildinfo:build-info-extractor-gradle:4.7.1")
		classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:$kotlinVersion")
		classpath("org.jetbrains.kotlin:kotlin-allopen:$kotlinVersion")
		classpath("io.spring.gradle:dependency-management-plugin:$springDependencyManagementVersion")
	}
}

val jvmEncoding = Charset.defaultCharset().name()
if (jvmEncoding != "UTF-8") {
	throw IllegalStateException("Build environment must be UTF-8 (it is: $jvmEncoding) - add '-Dfile.encoding=UTF-8' to the GRADLE_OPTS environment variable ")
}

if (!JavaVersion.current().isJava11Compatible) {
	throw IllegalStateException("Must be built with Java 11")
}

subprojects {
	group = "demo"
	val springDependencyManagementVersion: String by project

	apply(plugin = "java")
	apply(plugin = "groovy")
	apply(plugin = "idea")
	apply(plugin = "kotlin")
//	apply(plugin = "com.github.ben-manes.versions")
	apply(plugin = "io.spring.dependency-management")

	repositories {
		mavenLocal()
		mavenCentral()
		maven { url = uri("https://plugins.gradle.org/m2/") }
	}

//	the<DependencyManagementExtension>().apply {
//		imports {
//			mavenBom(org.springframework.boot.gradle.plugin.SpringBootPlugin.BOM_COORDINATES)
//		}
//	}
}
