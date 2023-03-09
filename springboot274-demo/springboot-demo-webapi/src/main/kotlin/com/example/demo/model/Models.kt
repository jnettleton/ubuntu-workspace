package com.example.demo.model

// import java.time.LocalDateTime
// import javax.persistence.*
//
// @Entity
// class Message(
// 		var title: String,
// 		var headline: String,
// 		var content: String,
// 		@ManyToOne var author: User,
// 		var slug: String = title.toSlug(),
// 		var addedAt: LocalDateTime = LocalDateTime.now(),
// 		@Id @GeneratedValue var id: Long? = null)

data class NFT (
    val id: Int,
    var name: String,
    var floor_price: Double
)
