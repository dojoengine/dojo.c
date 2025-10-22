/*
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo Go bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

package main

import (
	"fmt"
	"os"

	dojo "github.com/dojoengine/dojo.c/bindings/go"
)

func fetchEntities(toriiURL string, worldAddress string) error {
	fmt.Printf("Connecting to Torii at %s...\n", toriiURL)

	// Create a client with default 4MB max message size
	client, err := dojo.ToriiClientNewWithConfig(toriiURL, 4*1024*1024)
	if err != nil {
		return fmt.Errorf("failed to create client: %w", err)
	}
	defer client.Destroy()

	fmt.Println("✓ Connected successfully!")

	// Create pagination settings
	pagination := dojo.Pagination{
		Cursor:    nil, // Start from beginning
		Limit:     10,  // Fetch 10 entities
		Direction: dojo.PaginationDirectionForward,
		OrderBy:   []dojo.OrderBy{},
	}

	// Create a query to fetch entities
	query := dojo.Query{
		WorldAddresses: []string{},
		Pagination:     pagination,
		Clause:         nil, // No filtering clause
		NoHashedKeys:   false,
		Models:         []string{}, // Empty means all models
		Historical:     false,
	}

	fmt.Println("\nFetching entities...")
	page, err := client.Entities(query)
	if err != nil {
		return fmt.Errorf("failed to fetch entities: %w", err)
	}

	fmt.Printf("\n✓ Retrieved %d entities\n", len(page.Items))

	if page.NextCursor != nil {
		cursorStr := *page.NextCursor
		displayLen := 20
		if len(cursorStr) < displayLen {
			displayLen = len(cursorStr)
		}
		fmt.Printf("Next cursor available: %s...\n", cursorStr[:displayLen])
	} else {
		fmt.Println("No more pages available")
	}

	// Display entity information
	for i, entity := range page.Items {
		fmt.Println("\n" + repeatString("=", 60))
		fmt.Printf("Entity %d:\n", i+1)
		fmt.Printf("  World Address: %s\n", entity.WorldAddress)
		fmt.Printf("  Hashed Keys:   %s\n", entity.HashedKeys)
		fmt.Printf("  Created At:    %d\n", entity.CreatedAt)
		fmt.Printf("  Updated At:    %d\n", entity.UpdatedAt)
		fmt.Printf("  Executed At:   %d\n", entity.ExecutedAt)
		fmt.Printf("  Models:        %d model(s)\n", len(entity.Models))

		// Display model information
		for j, model := range entity.Models {
			fmt.Printf("\n  Model %d: %s\n", j+1, model.Name)
			fmt.Printf("    Children: %d field(s)\n", len(model.Children))

			displayCount := 3
			if len(model.Children) < displayCount {
				displayCount = len(model.Children)
			}

			for k := 0; k < displayCount; k++ {
				child := model.Children[k]
				fmt.Printf("      - %s (key=%t)\n", child.Name, child.Key)
			}

			if len(model.Children) > 3 {
				fmt.Printf("      ... and %d more\n", len(model.Children)-3)
			}
		}
	}

	fmt.Println("\n" + repeatString("=", 60) + "\n")

	return nil
}

func repeatString(s string, count int) string {
	result := ""
	for i := 0; i < count; i++ {
		result += s
	}
	return result
}

func main() {
	// Default Torii URL (adjust as needed)
	toriiURL := "http://localhost:8080"

	// Example world address (adjust to your actual world address)
	worldAddress := "0x0"

	// Check for command line arguments
	if len(os.Args) > 1 {
		toriiURL = os.Args[1]
	}
	if len(os.Args) > 2 {
		worldAddress = os.Args[2]
	}

	fmt.Println("Dojo Go Example: Fetch Entities")
	fmt.Println("================================\n")

	err := fetchEntities(toriiURL, worldAddress)
	if err != nil {
		fmt.Printf("\n❌ Error: %v\n", err)
		fmt.Printf("\nUsage: %s [torii_url] [world_address]\n", os.Args[0])
		fmt.Printf("Example: %s http://localhost:8080 0x1234...\n", os.Args[0])
		os.Exit(1)
	}
}
