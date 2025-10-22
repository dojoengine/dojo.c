/**
 * Simple test to verify Kotlin bindings work
 * 
 * This demonstrates basic connectivity to a Torii server.
 * Full examples with complex queries will work once the UniFFI
 * Kotlin generator better supports recursive types.
 */

package com.dojoengine.examples

import com.dojoengine.dojo.*

fun simpleTest(toriiUrl: String) {
    println("Testing Kotlin bindings...")
    println("Torii URL: $toriiUrl")
    
    try {
        // Create a client with default configuration
        val client = ToriiClient(toriiUrl)
        
        println("✓ Client created successfully!")
        println("✓ Kotlin bindings are working!")
        
        // Note: Full query examples will be available once UniFFI better supports
        // complex recursive types in Kotlin bindings
        println("\nNote: Complex queries with Clause, Primitive types etc. are currently")
        println("limited by UniFFI's Kotlin generator for recursive types.")
        println("Consider using Swift or Python bindings for full functionality.")
        
    } catch (e: DojoException.ConnectionError) {
        println("✗ Connection error: ${e.message}")
        println("  Make sure Torii server is running at $toriiUrl")
    } catch (e: Exception) {
        println("✗ Error: ${e.message}")
        e.printStackTrace()
    }
}

fun main(args: Array<String>) {
    val toriiUrl = args.getOrNull(0) ?: "http://localhost:8080"
    
    println("Dojo Kotlin Simple Test")
    println("========================\n")
    
    simpleTest(toriiUrl)
}

