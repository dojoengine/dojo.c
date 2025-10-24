/**
 * Example: Fetch entities from a Torii server
 *
 * This example demonstrates how to use the Dojo C bindings to:
 * 1. Connect to a Torii server
 * 2. Query entities with pagination
 * 3. Display entity data
 */

#include "../../bindings/c/dojo.h"
#include <stdio.h>
#include <string.h>

void print_separator(void) {
    printf("============================================================\n");
}

void print_field_element_hex(const char* label, struct FieldElement felt) {
    printf("%s0x", label);
    for (size_t i = 0; i < 32; i++) {
        printf("%02x", felt.data[i]);
    }
    printf("\n");
}

void fetch_entities(const char* torii_url, const char* world_address) {
    printf("Connecting to Torii at %s...\n", torii_url);
    
    // Create a client
    struct ResultToriiClient res_client = client_new(torii_url);
    if (res_client.tag == ErrToriiClient) {
        printf("Failed to create client: %s\n", res_client.err.message);
        error_free(&res_client.err);
        return;
    }
    struct ToriiClient* client = res_client.ok;
    
    printf("✓ Connected successfully!\n");
    
    // Create pagination settings
    struct Pagination pagination = {
        .cursor = { .tag = Nonec_char },           // Start from beginning
        .limit = { .tag = Someu32, .some = 10 },   // Fetch 10 entities
        .direction = Forward,                       // Forward pagination
        .order_by = { .data = NULL, .data_len = 0 } // No specific ordering
    };
    
    // Create a query to fetch entities
    struct Query query = {
        .world_addresses = { .data = NULL, .data_len = 0 },
        .pagination = pagination,
        .clause = { .tag = NoneClause },            // No filtering clause
        .no_hashed_keys = false,
        .models = { .data = NULL, .data_len = 0 },  // Empty means all models
        .historical = false
    };
    
    printf("\nFetching entities...\n");
    struct ResultPageEntity res_page = client_entities(client, query);
    if (res_page.tag == ErrPageEntity) {
        printf("Failed to fetch entities: %s\n", res_page.err.message);
        error_free(&res_page.err);
        client_free(client);
        return;
    }
    
    struct PageEntity page = res_page.ok;
    printf("\n✓ Retrieved %zu entities\n", page.items.data_len);
    
    if (page.next_cursor.tag == Somec_char) {
        // Show first 20 chars of cursor
        size_t cursor_len = strlen(page.next_cursor.some);
        size_t preview_len = cursor_len < 20 ? cursor_len : 20;
        printf("Next cursor available: %.*s...\n", (int)preview_len, page.next_cursor.some);
    } else {
        printf("No more pages available\n");
    }
    
    // Display entity information
    for (size_t i = 0; i < page.items.data_len; i++) {
        struct Entity* entity = &page.items.data[i];
        
        printf("\n");
        print_separator();
        printf("Entity %zu:\n", i + 1);
        print_field_element_hex("  World Address: ", entity->world_address);
        print_field_element_hex("  Hashed Keys:   ", entity->hashed_keys);
        printf("  Created At:    %llu\n", (unsigned long long)entity->created_at);
        printf("  Updated At:    %llu\n", (unsigned long long)entity->updated_at);
        printf("  Executed At:   %llu\n", (unsigned long long)entity->executed_at);
        printf("  Models:        %zu model(s)\n", entity->models.data_len);
        
        // Display model information
        for (size_t j = 0; j < entity->models.data_len; j++) {
            struct Struct* model = &entity->models.data[j];
            
            printf("\n  Model %zu: %s\n", j + 1, model->name);
            printf("    Children: %zu field(s)\n", model->children.data_len);
            
            // Show first 3 fields
            size_t fields_to_show = model->children.data_len < 3 ? model->children.data_len : 3;
            for (size_t k = 0; k < fields_to_show; k++) {
                struct Member* child = &model->children.data[k];
                printf("      - %s (key=%s)\n", child->name, child->key ? "true" : "false");
            }
            
            if (model->children.data_len > 3) {
                printf("      ... and %zu more\n", model->children.data_len - 3);
            }
        }
    }
    
    printf("\n");
    print_separator();
    printf("\n");
    
    // Clean up
    client_free(client);
}

int main(int argc, char* argv[]) {
    // Parse command line arguments
    const char* torii_url = (argc > 1) ? argv[1] : "http://localhost:8080";
    const char* world_address = (argc > 2) ? argv[2] : "0x0";
    
    printf("Dojo C Example: Fetch Entities\n");
    printf("====================================\n\n");
    
    // Run the function
    fetch_entities(torii_url, world_address);
    
    return 0;
}