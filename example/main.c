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
    EntityQuery entity = {};
    entity.model = "Moves";
    entity.clause.keys.data = malloc(sizeof(FieldElement));
    entity.clause.keys.data_len = 1;

    if (hex_to_bytes(player, entity.clause.keys.data->data) == 0) {
        for (size_t i = 0; i < 32; i++) {
            printf("%02x", entity.clause.keys.data->data[i]);
        }
        printf("\n");
    } else {
        printf("Invalid hex string\n");
    }

    Ty *ty = client_entity(client, &entity, &error);

    if (ty == NULL)
    {
        printf("Failed to get entity: %s\n", error.message);
        return 1;
    }

    ty_free(ty);

    // printf("Got entity\n");
    // printf("Struct: %s\n", ty->ty_struct.name);
    // printf("Fields: %s\n", ty->ty_struct.children.data[2].name);
    // printf("Enum: %s\n", ty->ty_struct.children.data[2].ty->ty_enum.options.data[0].name);

    // client_on_entity_state_update(client, &entity, &on_entity_state_update, &error);
    // while (true)
    // {
    // }

    // Remember to free the client when you're done with it.
    client_free(client);

    return 0;
}