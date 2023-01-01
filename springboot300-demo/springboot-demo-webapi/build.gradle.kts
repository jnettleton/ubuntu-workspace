import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.springframework.boot.gradle.tasks.bundling.BootBuildImage
import org.jetbrains.kotlin.allopen.gradle.AllOpenExtension

val kotlinVersion: String by project
val kotlinLoggingVersion: String by project
val logbackVersion: String by project

plugins {
	id("org.springframework.boot") version "3.0.0
	id("io.spring.dependency-management") version "1.1.0"
	kotlin("jvm") version "1.7.21"
	kotlin("plugin.spring") version "1.7.21"
	id("org.jetbrains.kotlin.plugin.allopen")
}

group = "com.example.demo"
version = "0.0.1-SNAPSHOT"
java.sourceCompatibility = JavaVersion.VERSION_17

configure<AllOpenExtension> {
	annotation("org.springframework.context.annotation.Configuration")
}

repositories {
	mavenCentral()
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter-web")
	implementation("org.springframework.boot:spring-boot-starter")
	implementation("org.jetbrains.kotlin:kotlin-reflect")
	implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
	developmentOnly("org.springframework.boot:spring-boot-devtools")
	annotationProcessor("org.springframework.boot:spring-boot-configuration-processor")
	testImplementation("org.springframework.boot:spring-boot-starter-test")

	// JSON
	implementation("com.fasterxml.jackson.module:jackson-module-kotlin")

	// Logging
	implementation("io.github.microutils:kotlin-logging:$kotlinLoggingVersion")
	implementation("ch.qos.logback:logback-classic:$logbackVersion")
	implementation("ch.qos.logback:logback-core:$logbackVersion")
	implementation("net.logstash.logback:logstash-logback-encoder:7.2")
}

tasks.withType<KotlinCompile> {
	kotlinOptions {
		freeCompilerArgs = listOf("-Xjsr305=strict")
		jvmTarget = "11"
	}
}

tasks.withType<Test> {
	useJUnitPlatform()
}

//tasks.withType<BootBuildImage> {
//	builder = "paketobuildpacks/builder:tiny"
//	environment = mapOf("BP_NATIVE_IMAGE" to "true")
//}
