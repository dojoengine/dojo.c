/**
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo Swift bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

import Foundation

func fetchEntities(toriiUrl: String, worldAddress: String) async throws {
        print("Connecting to Torii at \(toriiUrl)...")
        
        // Create a client with default configuration (4MB max message size)
        let client = try await ToriiClient(toriiUrl: toriiUrl)
        
        print("✓ Connected successfully!")
        
        // Create a query to fetch entities
        let query = Query(
            worldAddresses: [],
            pagination: Pagination(
                cursor: nil,          // Start from beginning
                limit: 10,            // Fetch 10 entities
                direction: .forward,  // Forward pagination
                orderBy: []           // No specific ordering
            ),
            clause: nil,              // No filtering clause
            noHashedKeys: false,
            models: [],               // Empty means all models
            historical: false
        )
        
        print("\nFetching entities...")
        let page = try await client.entities(query: query)
        
        print("\n✓ Retrieved \(page.items.count) entities")
        
        if let nextCursor = page.nextCursor {
            let cursorPreview = String(nextCursor.prefix(20))
            print("Next cursor available: \(cursorPreview)...")
        } else {
            print("No more pages available")
        }
        
        // Display entity information
        for (i, entity) in page.items.enumerated() {
            print("\n" + String(repeating: "=", count: 60))
            print("Entity \(i + 1):")
            print("  World Address: \(entity.worldAddress)")
            print("  Hashed Keys:   \(entity.hashedKeys)")
            print("  Created At:    \(entity.createdAt)")
            print("  Updated At:    \(entity.updatedAt)")
            print("  Executed At:   \(entity.executedAt)")
            print("  Models:        \(entity.models.count) model(s)")
            
            // Display model information
            for (j, model) in entity.models.enumerated() {
                print("\n  Model \(j + 1): \(model.name)")
                print("    Children: \(model.children.count) field(s)")
                
                // Show first 3 fields
                for child in model.children.prefix(3) {
                    print("      - \(child.name) (key=\(child.key))")
                }
                
                if model.children.count > 3 {
                    print("      ... and \(model.children.count - 3) more")
                }
            }
        }
        
        print("\n" + String(repeating: "=", count: 60) + "\n")
}

// Parse command line arguments
let arguments = CommandLine.arguments
let toriiUrl = arguments.count > 1 ? arguments[1] : "http://localhost:8080"
let worldAddress = arguments.count > 2 ? arguments[2] : "0x0"

print("Dojo Swift Example: Fetch Entities")
print("====================================\n")

// Run the async function
Task {
    do {
        try await fetchEntities(toriiUrl: toriiUrl, worldAddress: worldAddress)
        exit(0)
    } catch {
        print("\n❌ Error: \(error)")
        print("\nUsage: swift fetch_entities.swift [torii_url] [world_address]")
        print("Example: swift fetch_entities.swift http://localhost:8080 0x1234...")
        exit(1)
    }
}

// Keep the program running until the task completes
RunLoop.main.run()

