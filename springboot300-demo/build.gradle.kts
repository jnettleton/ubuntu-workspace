import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
	id("org.springframework.boot") version "3.0.0"
	id("io.spring.dependency-management") version "1.1.0"
	kotlin("jvm") version "1.7.21"
	kotlin("plugin.spring") version "1.7.21"
}

group = "com.example"
version = "0.0.1-SNAPSHOT"
java.sourceCompatibility = JavaVersion.VERSION_17

repositories {
	mavenCentral()
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter")
	implementation("org.jetbrains.kotlin:kotlin-reflect")
	implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
	developmentOnly("org.springframework.boot:spring-boot-devtools")
	testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.withType<KotlinCompile> {
	kotlinOptions {
		freeCompilerArgs = listOf("-Xjsr305=strict")
		jvmTarget = "17"
	}
}

tasks.withType<Test> {
	useJUnitPlatform()
}

subprojects {
	group = "spingboot-demo-webapi"
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
