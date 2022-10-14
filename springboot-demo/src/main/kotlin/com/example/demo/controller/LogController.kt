package com.example.demo.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RestController
import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.ResponseStatus

import com.example.demo.model.NFT
import com.example.demo.exception.LogNotFoundException
import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper

@RestController
class LogController {
    private var NFTs = mutableListOf(
        NFT(1, "CryptoPunks", 100.0),
        NFT(2, "Sneaky Vampire Syndicate", 36.9),
        NFT(3, "The Sevens (Official)", 0.6),
        NFT(4, "Art Blocks Curated", 1.1),
        NFT(5, "Pudgy Penguins", 2.5),
    )

    private val x: Int;
    private val sumoLogs: HashMap<String, String>

    constructor() {
        println("primary constructor")
        x = 5;

        // create list of valid log names from "/etc/sumo-sources.json"
        sumoLogs = HashMap<String, String>()
        val fullJsonContent = """
        {
            "api.version": "v1",
            "sources": [
                {
                    "name": "demo-log",
                    "hostName": "demo-host",
                    "category": "demo-dev",
                    "automaticDateParsing": true,
                    "multilineProcessingEnabled": true,
                    "useAutolineMatching": false,
                    "manualPrefixRegexp": "timestamp=.*",
                    "forceTimeZone": false,
                    "timeZone": "UTC",
                    "filters": [
                        {
                            "filterType": "Exclude",
                            "name": "Exclude TRACE level",
                            "regexp": "(?s).*TRACE(?s).*"
                        },
                        {
                            "filterType": "Exclude",
                            "name": "Exclude DEBUG level",
                            "regexp": "(?s).*DEBUG(?s).*"
                        }
                    ],
                    "encoding": "UTF-8",
                    "pathExpression": "/var/log/demo/demoLog.log",
                    "blacklist": [
                    ],
                    "sourceType": "LocalFile",
                    "alive": true
                }
            ]
        }""".trimIndent()

        // parse "
        val mapper = jacksonObjectMapper()
        val root: JsonNode = mapper.readTree(fullJsonContent)
        val sourcesNode: JsonNode = root.get("sources")
        for (sourceNode: JsonNode in sourcesNode) {
            var fullPath = sourceNode.get("pathExpression").asText()
            val fullName = fullPath.substringAfterLast("/")
            val fileName = fullName.substringBeforeLast(".").lowercase()

            sumoLogs.put(fileName, fullName)
        }

    }

    @GetMapping("/")
    //@RequestMapping(method = arrayOf(RequestMethod.GET))
    fun hello() : String {
      return "Hello, World!"
    }

    @GetMapping("/log/{name}")
	fun helloName(@PathVariable name:String) : String {
		return "Hello Log, ${name}"
    }

    @PostMapping("/log/{name}") // 1
    @ResponseStatus(HttpStatus.CREATED) // 2
    fun postLogMessages(@PathVariable name:String, @RequestBody nft: NFT): NFT { // 3
        if (!sumoLogs.containsKey(name.lowercase())) throw LogNotFoundException()

        val maxId = NFTs.map { it.id }.maxOrNull() ?: 0 // 4
        val nextId = maxId + 1 // 5
        val newNft = NFT(id = nextId, name = nft.name, floor_price = nft.floor_price) // 6
        NFTs.add(newNft) // 7
        return newNft
    }
}