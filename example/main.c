#include "../dojo.h"
#include <stdio.h>

int main()
{
    const char *torii_url = "http://localhost:8080";
    const char *rpc_url = "http://localhost:5050";

    const char* playerKey = "517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973", *pos = playerKey;
    const char* world = "0x5010c31f127114c6198df8a5239e2b7a5151e1156fb43791e37e7385faa8138";
    // Initialize world.data here...

    EntityQuery entities[1] = {};
    // Initialize entities[0].model, entities[0].keys, and entities[0].keys_len here...
    entities[0].model = "Moves";

    Error error;

    ToriiClient *client = client_new(torii_url, rpc_url, world, entities, 1, &error);

    if (client == NULL)
    {
        printf("Failed to create client: %s\n", error.message);
        return 1;
    }

    // Use client here...
    WorldMetadata metadata = client_metadata(client);
    printf("World metadata:\n");
    printf("  world_address: %");
    for (int i = 0; i < 32; i++)
    {
        printf("%02x", metadata.world_address.data[i]);
    }
    printf("\n");
    



    // Remember to free the client when you're done with it.
    client_free(client);

    return 0;
}