/**
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo C++ bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include "../../bindings/cpp/dojo.hpp"

void fetchEntities(const std::string& toriiUrl, const std::string& worldAddress) {
    std::cout << "Connecting to Torii at " << toriiUrl << "..." << std::endl;
    
    // Create a client with default configuration (4MB max message size)
    auto client = dojo::ToriiClient::init(toriiUrl);
    
    std::cout << "✓ Connected successfully!" << std::endl;
    
    // Create pagination settings
    auto pagination = std::make_shared<dojo::Pagination>();
    pagination->cursor = std::nullopt;        // Start from beginning
    pagination->limit = 10;                    // Fetch 10 entities
    pagination->direction = dojo::PaginationDirection::kForward;  // Forward pagination
    pagination->order_by = {};                 // No specific ordering
    
    // Create a query to fetch entities
    dojo::Query query;
    query.world_addresses = {};
    query.pagination = pagination;
    query.clause = std::nullopt;               // No filtering clause
    query.no_hashed_keys = false;
    query.models = {};                         // Empty means all models
    query.historical = false;
    
    std::cout << "\nFetching entities..." << std::endl;
    auto page = client->entities(query);
    
    std::cout << "\n✓ Retrieved " << page.items.size() << " entities" << std::endl;
    
    if (page.next_cursor) {
        std::string cursorPreview = page.next_cursor.value().substr(0, std::min<size_t>(20, page.next_cursor.value().length()));
        std::cout << "Next cursor available: " << cursorPreview << "..." << std::endl;
    } else {
        std::cout << "No more pages available" << std::endl;
    }
    
    // Display entity information
    for (size_t i = 0; i < page.items.size(); ++i) {
        const auto& entity = page.items[i];
        
        std::cout << "\n" << std::string(60, '=') << std::endl;
        std::cout << "Entity " << (i + 1) << ":" << std::endl;
        std::cout << "  World Address: " << entity->world_address << std::endl;
        std::cout << "  Hashed Keys:   " << entity->hashed_keys << std::endl;
        std::cout << "  Created At:    " << entity->created_at << std::endl;
        std::cout << "  Updated At:    " << entity->updated_at << std::endl;
        std::cout << "  Executed At:   " << entity->executed_at << std::endl;
        std::cout << "  Models:        " << entity->models.size() << " model(s)" << std::endl;
        
        // Display model information
        for (size_t j = 0; j < entity->models.size(); ++j) {
            const auto& model = entity->models[j];
            
            std::cout << "\n  Model " << (j + 1) << ": " << model->name << std::endl;
            std::cout << "    Children: " << model->children.size() << " field(s)" << std::endl;
            
            // Show first 3 fields
            size_t fieldsToShow = std::min<size_t>(3, model->children.size());
            for (size_t k = 0; k < fieldsToShow; ++k) {
                const auto& child = model->children[k];
                std::cout << "      - " << child->name << " (key=" << child->key << ")" << std::endl;
            }
            
            if (model->children.size() > 3) {
                std::cout << "      ... and " << (model->children.size() - 3) << " more" << std::endl;
            }
        }
    }
    
    std::cout << "\n" << std::string(60, '=') << "\n" << std::endl;
}

int main(int argc, char* argv[]) {
    // Parse command line arguments
    std::string toriiUrl = (argc > 1) ? argv[1] : "http://localhost:8080";
    std::string worldAddress = (argc > 2) ? argv[2] : "0x0";
    
    std::cout << "Dojo C++ Example: Fetch Entities" << std::endl;
    std::cout << "====================================" << std::endl << std::endl;
    
    // Run the function
    try {
        fetchEntities(toriiUrl, worldAddress);
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "\n❌ Error: " << e.what() << std::endl;
        std::cerr << "\nUsage: " << argv[0] << " [torii_url] [world_address]" << std::endl;
        std::cerr << "Example: " << argv[0] << " http://localhost:8080 0x1234..." << std::endl;
        return 1;
    }
}
