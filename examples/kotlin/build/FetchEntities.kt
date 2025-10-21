/**
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo Kotlin bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

package com.dojoengine.examples

import com.dojoengine.dojo.*

fun fetchEntities(toriiUrl: String, worldAddress: String) {
    println("Connecting to Torii at $toriiUrl...")
    
    // Create a client with default configuration (4MB max message size)
    val client = ToriiClient(toriiUrl)
    
    println("✓ Connected successfully!")
    
    // Create a query to fetch entities
    val query = Query(
        worldAddresses = emptyList(),
        pagination = Pagination(
            cursor = null,          // Start from beginning
            limit = 10u,            // Fetch 10 entities
            direction = PaginationDirection.FORWARD,
            orderBy = emptyList()   // No specific ordering
        ),
        clause = null,              // No filtering clause
        noHashedKeys = false,
        models = emptyList(),       // Empty means all models
        historical = false
    )
    
    println("\nFetching entities...")
    val page = client.entities(query)
    
    println("\n✓ Retrieved ${page.items.size} entities")
    
    if (page.nextCursor != null) {
        val cursorPreview = page.nextCursor.take(20)
        println("Next cursor available: $cursorPreview...")
    } else {
        println("No more pages available")
    }
    
    // Display entity information
    page.items.forEachIndexed { i, entity ->
        println("\n${"=".repeat(60)}")
        println("Entity ${i + 1}:")
        println("  World Address: ${entity.worldAddress}")
        println("  Hashed Keys:   ${entity.hashedKeys}")
        println("  Created At:    ${entity.createdAt}")
        println("  Updated At:    ${entity.updatedAt}")
        println("  Executed At:   ${entity.executedAt}")
        println("  Models:        ${entity.models.size} model(s)")
        
        // Display model information
        entity.models.forEachIndexed { j, model ->
            println("\n  Model ${j + 1}: ${model.name}")
            println("    Children: ${model.children.size} field(s)")
            
            // Show first 3 fields
            model.children.take(3).forEach { child ->
                println("      - ${child.name} (key=${child.key})")
            }
            
            if (model.children.size > 3) {
                println("      ... and ${model.children.size - 3} more")
            }
        }
    }
    
    println("\n${"=".repeat(60)}\n")
}

fun main(args: Array<String>) {
    val toriiUrl = args.getOrNull(0) ?: "http://localhost:8080"
    val worldAddress = args.getOrNull(1) ?: "0x0"
    
    println("Dojo Kotlin Example: Fetch Entities")
    println("====================================\n")
    
    try {
        fetchEntities(toriiUrl, worldAddress)
    } catch (e: Exception) {
        println("\n❌ Error: ${e.message}")
        println("\nUsage: kotlin FetchEntities.kt [torii_url] [world_address]")
        println("Example: kotlin FetchEntities.kt http://localhost:8080 0x1234...")
        System.exit(1)
    }
}

