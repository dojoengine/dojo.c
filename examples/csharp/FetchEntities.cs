/*
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo C# bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

using System;
using System.Threading.Tasks;
using uniffi.dojo;  // Generated bindings namespace

namespace DojoExamples
{
    class FetchEntities
    {
        static async Task FetchEntitiesExample(string toriiUrl, string worldAddress)
        {
            Console.WriteLine($"Connecting to Torii at {toriiUrl}...");
            
            // Create a client with default 4MB max message size
            var client = await ToriiClient.NewWithConfig(toriiUrl, 4 * 1024 * 1024);
            
            Console.WriteLine("✓ Connected successfully!");
            
            // Create a query to fetch entities
            var query = new Query(
                worldAddresses: new string[] { },
                pagination: new Pagination(
                    cursor: null,  // Start from beginning
                    limit: 10,     // Fetch 10 entities
                    direction: PaginationDirection.Forward,
                    orderBy: new OrderBy[] { }
                ),
                clause: null,  // No filtering clause
                noHashedKeys: false,
                models: new string[] { },  // Empty means all models
                historical: false
            );
            
            Console.WriteLine("\nFetching entities...");
            var page = await client.Entities(query);
            
            Console.WriteLine($"\n✓ Retrieved {page.Items.Length} entities");
            
            if (page.NextCursor != null)
            {
                Console.WriteLine($"Next cursor available: {page.NextCursor.Substring(0, Math.Min(20, page.NextCursor.Length))}...");
            }
            else
            {
                Console.WriteLine("No more pages available");
            }
            
            // Display entity information
            for (int i = 0; i < page.Items.Length; i++)
            {
                var entity = page.Items[i];
                Console.WriteLine($"\n{new string('=', 60)}");
                Console.WriteLine($"Entity {i + 1}:");
                Console.WriteLine($"  World Address: {entity.WorldAddress}");
                Console.WriteLine($"  Hashed Keys:   {entity.HashedKeys}");
                Console.WriteLine($"  Created At:    {entity.CreatedAt}");
                Console.WriteLine($"  Updated At:    {entity.UpdatedAt}");
                Console.WriteLine($"  Executed At:   {entity.ExecutedAt}");
                Console.WriteLine($"  Models:        {entity.Models.Length} model(s)");
                
                // Display model information
                for (int j = 0; j < entity.Models.Length; j++)
                {
                    var model = entity.Models[j];
                    Console.WriteLine($"\n  Model {j + 1}: {model.Name}");
                    Console.WriteLine($"    Children: {model.Children.Length} field(s)");
                    
                    int displayCount = Math.Min(3, model.Children.Length);
                    for (int k = 0; k < displayCount; k++)
                    {
                        var child = model.Children[k];
                        Console.WriteLine($"      - {child.Name} (key={child.Key})");
                    }
                    
                    if (model.Children.Length > 3)
                    {
                        Console.WriteLine($"      ... and {model.Children.Length - 3} more");
                    }
                }
            }
            
            Console.WriteLine($"\n{new string('=', 60)}\n");
        }
        
        static async Task Main(string[] args)
        {
            // Default Torii URL (adjust as needed)
            string toriiUrl = "http://localhost:8080";
            
            // Example world address (adjust to your actual world address)
            string worldAddress = "0x0";
            
            // Check for command line arguments
            if (args.Length > 0)
            {
                toriiUrl = args[0];
            }
            if (args.Length > 1)
            {
                worldAddress = args[1];
            }
            
            Console.WriteLine("Dojo C# Example: Fetch Entities");
            Console.WriteLine("================================\n");
            
            try
            {
                await FetchEntitiesExample(toriiUrl, worldAddress);
            }
            catch (Exception e)
            {
                Console.WriteLine($"\n❌ Error: {e.Message}");
                Console.WriteLine($"\nUsage: dotnet run [torii_url] [world_address]");
                Console.WriteLine($"Example: dotnet run http://localhost:8080 0x1234...");
                Environment.Exit(1);
            }
        }
    }
}

