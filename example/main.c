#include "../dojo.h"
#include <stdio.h>

int main()
{
    const char *torii_url = "http://localhost:50051";
    const char *rpc_url = "http://localhost:50052";

    FieldElement world;
    // Initialize world.data here...

    EntityQuery entities[1];
    // Initialize entities[0].model, entities[0].keys, and entities[0].keys_len here...

    Error error;

    Client *client = client_new(torii_url, rpc_url, &world, entities, 1, &error);

    if (client == NULL)
    {
        printf("Failed to create client: %s\n", error.message);
        return 1;
    }

    client->entity(client->client, &entities[0], &error);
    if (error.message != NULL)
    {
        printf("Failed to get entity: %s\n", error.message);
        return 1;
    }

    // Use client here...

    // Remember to free the client when you're done with it.
    client->free(client->client);

    return 0;
}