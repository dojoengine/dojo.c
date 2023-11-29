#include "../dojo.h"
#include <stdio.h>

void on_entity_state_update()
{
    printf("on_entity_state_update\n");
}

int hex_to_bytes(const char *hex, unsigned char *bytes)
{

    if (hex[0] == '0' && hex[1] == 'x')
    {
        hex += 2;
    }

    for (size_t i = 0; i < 32; i++)
    {
        sscanf(hex + 2 * i, "%2hhx", &bytes[i]);
    }

    return 0;
}

int main()
{
    const char *torii_url = "http://localhost:8080";
    const char *rpc_url = "http://localhost:5050";

    const char *player = "0x0517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973";
    const char *world = "0x05010c31f127114c6198df8a5239e2b7a5151e1156fb43791e37e7385faa8138";
    // Initialize world.data here...

    Keys entities[1] = {};
    // Initialize entities[0].model, entities[0].keys, and entities[0].keys_len here...
    entities[0].model = "Moves";
    entities[0].keys.data = malloc(sizeof(char*));
    entities[0].keys.data_len = 1;

    entities[0].keys.data[0] = player;

    Error error;
    ToriiClient *client = client_new(torii_url, rpc_url, world, entities, 1, &error);
    if (client == NULL)
    {
        printf("Failed to create client: %s\n", error.message);
        return 1;
    }

    error = (Error){};
    Ty *ty = client_entity(client, entities, &error);
    if (ty == NULL)
    {
        printf("Failed to get entity: %s\n", error.message);
        return 1;
    }

    // printf("model: %s\n", entities[0].model);

    printf("Got entity\n");
    printf("Struct: %s\n", ty->ty_struct.name);
    printf("Fields: %s\n", ty->ty_struct.children.data[2].name);
    printf("Enum: %s\n", ty->ty_struct.children.data[2].ty->ty_enum.name);

    CArray_KeysClause* subscribed_entities = client_subscribed_entities(client);
    for (size_t i = 0; i < subscribed_entities->data_len; i++)
    {
        // print player key
        const char* hex;
        printf("Subscribed entity: ");
        for (size_t j = 0; j < 32; j++) {
            printf("%02x", subscribed_entities->data[i].keys.data->data[j]);
        }
        printf("\n");
    }

    client_add_entities_to_sync(client, entities, 1, &error);

    client_on_entity_state_update(client, entities, &on_entity_state_update);
    while (true)
    {
    }

    ty_free(ty);

    // Remember to free the client when you're done with it.
    client_free(client);

    return 0;
}