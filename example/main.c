#include "../dojo.h"
#include <unistd.h>
#include <stdio.h>

void on_entity_state_update(FieldElement key, CArrayModel models)
{
    printf("on_entity_state_update\n");
    printf("Key: 0x");
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", key.data[i]);
    }
    printf("\n");

    for (size_t i = 0; i < models.data_len; i++)
    {
        printf("Model: %s\n", models.data[i].name);
        // for (size_t j = 0; j < models.data[i].children.data_len; j++)
        // {
        //     printf("Field: %s\n", models.data[i].children.data[j].name);
        //     printf("Value: %s\n", models.data[i].children.data[j].value);
        // }
    }
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

    const char *playerKey = "0x028cd7ee02d7f6ec9810e75b930e8e607793b302445abbdee0ac88143f18da20";
    const char *playerAddress = "0x0517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973";
    const char *world = "0x028f5999ae62fec17c09c52a800e244961dba05251f5aaf923afabd9c9804d1a";
    const char *actions = "0x03e274b7d85fd0415bec56a5d831c3854f7308bbb26d486993cfc49e5a5fb788";
    // Initialize world.data here...

    KeysClause entities[1] = {};
    // Initialize entities[0].model, entities[0].keys, and entities[0].keys_len here...
    entities[0].model = "Position";
    entities[0].keys.data = malloc(sizeof(char *));
    entities[0].keys.data_len = 1;
    entities[0].keys.data[0] = playerKey;

    ResultToriiClient resClient = client_new(torii_url, rpc_url, world, entities, 1);
    if (resClient.tag == ErrAccount)
    {
        printf("Failed to create client: %s\n", resClient.err.message);
        return 1;
    }
    ToriiClient *client = resClient.ok;

    // signing key
    ResultFieldElement resSigningKey = felt_from_hex_be("0x1800000000300000180000000000030000000000003006001800006600");
    if (resSigningKey.tag == ErrFieldElement)
    {
        printf("Failed to create signing key: %s\n", resSigningKey.err.message);
        return 1;
    }
    FieldElement signing_key = resSigningKey.ok;

    // provider
    ResultCJsonRpcClient resProvider = jsonrpc_client_new(rpc_url);
    if (resProvider.tag == ErrCJsonRpcClient)
    {
        printf("Failed to create provider: %s\n", resProvider.err.message);
        return 1;
    }
    CJsonRpcClient *provider = resProvider.ok;

    // account
    ResultAccount resAccount = account_new(provider, signing_key, playerAddress);
    if (resAccount.tag == ErrAccount)
    {
        printf("Failed to create account: %s\n", resAccount.err.message);
        return 1;
    }
    Account *master_account = resAccount.ok;

    ResultAccount resBurner = account_deploy_burner(master_account);
    if (resBurner.tag == ErrAccount)
    {
        printf("Failed to create burner: %s\n", resBurner.err.message);
        return 1;
    }

    Account *burner = resBurner.ok;

    FieldElement address = account_address(burner);
    printf("New account: 0x");
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", address.data[i]);
    }
    printf("\n");

    ResultCOptionTy resTy = client_model(client, entities);
    if (resTy.tag == ErrCOptionTy)
    {
        printf("Failed to get entity: %s\n", resTy.err.message);
        return 1;
    }
    COptionTy ty = resTy.ok;

    if (ty.tag == SomeTy)
    {
        printf("Got entity\n");
        printf("Struct: %s\n", ty.some->struct_.name);
        for (size_t i = 0; i < ty.some->struct_.children.data_len; i++)
        {
            printf("Field: %s\n", ty.some->struct_.children.data[i].name);
        }

        ty_free(ty.some);
    }

    Query query = {};
    query.limit = 100;
    query.clause.tag = NoneClause;
    query.clause.some.tag = Keys;
    query.clause.some.keys.keys.data = malloc(sizeof(char *));
    query.clause.some.keys.keys.data_len = 1;
    query.clause.some.keys.keys.data[0] = playerAddress;
    query.clause.some.keys.model = "Moves";
    ResultCArrayEntity resEntities = client_entities(client, &query);
    if (resEntities.tag == ErrCArrayEntity)
    {
        printf("Failed to get entities: %s\n", resEntities.err.message);
        return 1;
    }

    CArrayEntity fetchedEntities = resEntities.ok;
    printf("Fetched %zu entities\n", fetchedEntities.data_len);
    for (size_t i = 0; i < fetchedEntities.data_len; i++)
    {
        // pritn hex of key
        printf("Key: 0x");
        for (size_t j = 0; j < 32; j++)
        {
            printf("%02x", fetchedEntities.data[i].hashed_keys.data[j]);
        }
        printf("\n");

        // print models name
        for (size_t j = 0; j < fetchedEntities.data[i].models.data_len; j++)
        {
            printf("Model: %s\n", fetchedEntities.data[i].models.data[j].name);
        }
    }

    // Result_bool resStartSub = client_start_subscription(client);
    // if (resStartSub.tag == Err_bool)
    // {
    //     printf("Failed to start subscription: %s\n", resStartSub.err.message);
    //     return 1;
    // }

    // Result_bool resAddEntities = client_add_models_to_sync(client, entities, 1);
    // if (resAddEntities.tag == Err_bool)
    // {
    //     printf("Failed to add entities to sync: %s\n", resAddEntities.err.message);
    //     return 1;
    // }

    // // print subscribed entities
    const CArrayKeysClause subscribed_models = client_subscribed_models(client);
    for (size_t i = 0; i < subscribed_models.data_len; i++)
    {
        printf("Subscribed entity: %s", subscribed_models.data[i].keys.data[0]);
        printf("\n");
    }

    FieldElement keys[1] = {};
    keys[0] = felt_from_hex_be(playerKey).ok;
    Resultbool resEntityUpdate = client_on_entity_state_update(client, keys, 0, &on_entity_state_update);
    if (resEntityUpdate.tag == Errbool)
    {
        printf("Failed to set entity update callback: %s\n", resEntityUpdate.err.message);
        return 1;
    }

    Call spawn = {
        .to = actions,
        .selector = "spawn",
    };

    Call move = {
        .to = actions,
        .selector = "move",
        .calldata = {
            .data = malloc(sizeof(FieldElement)),
            .data_len = 1,
        }};

    ResultFieldElement moveLeft = felt_from_hex_be("0x01");
    if (moveLeft.tag == ErrFieldElement)
    {
        printf("Failed to create moveLeft: %s\n", moveLeft.err.message);
        return 1;
    }

    move.calldata.data[0] = moveLeft.ok;

    ResultFieldElement resSpawn = account_execute_raw(master_account, &spawn, 1);
    if (resSpawn.tag == Errbool)
    {
        printf("Failed to execute call: %s\n", resSpawn.err.message);
        return 1;
    }
    wait_for_transaction(provider, resSpawn.ok);

    ResultFieldElement resMove = account_execute_raw(master_account, &move, 1);
    if (resMove.tag == Errbool)
    {
        printf("Failed to execute call: %s\n", resMove.err.message);
        return 1;
    }
    wait_for_transaction(provider, resMove.ok);

    // while (1)
    // {

    // }

    // Result_bool resRemoveEntities = client_remove_models_to_sync(client, entities, 1);
    // if (resRemoveEntities.tag == Err_bool)
    // {
    //     printf("Failed to remove entities to sync: %s\n", resRemoveEntities.err.message);
    //     return 1;
    // }

    // Remember to free the client when you're done with it.
    client_free(client);

    return 0;
}