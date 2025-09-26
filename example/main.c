#include "../dojo.h"
#include <unistd.h>
#include <stdio.h>
#include <string.h>

// Add this global variable near the top of the file
static struct SessionAccount* g_session_account = NULL;

void on_entity_state_update(FieldElement key, CArrayStruct models)
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

void hex_to_bytes(const char *hex, FieldElement *felt)
{
    if (hex[0] == '0' && hex[1] == 'x')
    {
        hex += 2;
    }

    // handle hex less than 64 characters - pad with 0
    size_t len = strlen(hex);
    if (len < 64)
    {
        char *padded = malloc(65);
        memset(padded, '0', 64 - len);
        padded[64 - len] = '\0';
        strcat(padded, hex);
        hex = padded;
    }

    for (size_t i = 0; i < 32; i++)
    {
        sscanf(hex + 2 * i, "%2hhx", &(*felt).data[i]);
    }
}

void on_account_connected(struct SessionAccount *account)
{
    // Store the account in our global variable
    g_session_account = account;
}

int main()
{
    const char *torii_url = "http://localhost:8080";
    const char *rpc_url = "http://localhost:5050";

    const char *player_address = "0x127fd5f1fe78a71f8bcd1fec63e3fe2f0486b6ecd5c86a0466c3a21fa5cfcec";
    const char *player_signing_key = "0xc5b2fcab997346f3ea1c00b002ecf6f382c5f9c9659a3894eb783c5320f912";
    FieldElement world;
    hex_to_bytes("0x01385f25d20a724edc9c7b3bd9636c59af64cbaf9fcd12f33b3af96b2452f295", &world);
    FieldElement actions;
    hex_to_bytes("0x04ba8772b4785c0afce5b73ed98d30cf8832e3bfcceff5a688b085ef6d0f164e", &actions);
    FieldElement eth;
    hex_to_bytes("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7", &eth);
    FieldElement katana;
    hex_to_bytes("0x03dc18a09d1dc893eb1abce2e0d33b8cc285ea430a4bdd30bccf0c8638e59659", &katana);

    ResultToriiClient resClient = client_new(torii_url, rpc_url, "/ip4/127.0.0.1/tcp/9090", world);
    if (resClient.tag == ErrToriiClient)
    {
        printf("Failed to create client: %s\n", resClient.err.message);
        return 1;
    }
    struct ToriiClient *client = resClient.ok;

    Policy policies[] = {
        {katana, "create_Board", "Create a board"},
        {katana, "create_Game", "Create a game"},
    };

    ResultProvider resControllerProvider = provider_new("https://api.cartridge.gg/x/knet-controller/katana");
    if (resControllerProvider.tag == ErrProvider)
    {
        printf("Failed to create provider: %s\n", resControllerProvider.err.message);
        return 1;
    }
    struct Provider *controller_provider = resControllerProvider.ok;

    ResultController resSessionAccount = controller_account(policies, 2);
    if (resSessionAccount.tag == OkController) {
        printf("Session account already connected\n");
        g_session_account = resSessionAccount.ok;
    } else {
        controller_connect("https://api.cartridge.gg/x/knet-controller/katana", policies, 2, on_account_connected);
    }
    
    // Wait for the account to be connected
    while (g_session_account == NULL) {
        usleep(100000); // Sleep for 100ms to avoid busy waiting
    }
    
    FieldElement controller_addr = controller_address(g_session_account);
    printf("Connected controller address: 0x");
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", controller_addr.data[i]);
    }
    printf("\n");

    // Transfer a bit of ETH to another account
    Call create_Game = {
        .to = "0x0589634122ff8b84021b3f683cbf9d5adf15482289e1d70b1f8ad3052cc08711",
        .selector = "move",
        .calldata = {
            .data = malloc(sizeof(FieldElement) * 2),
            .data_len = 2,
        }};

    
    hex_to_bytes("0x0", &create_Game.calldata.data[0]);
    hex_to_bytes("0x2", &create_Game.calldata.data[1]);

    // call send_transaction (rpc_url) 
    ResultFieldElement resCreateGame = controller_execute_from_outside(g_session_account, &create_Game, 1);
    if (resCreateGame.tag == ErrFieldElement)
    {
        printf("Failed to execute call: %s\n", resCreateGame.err.message);
    }

    wait_for_transaction(controller_provider, resCreateGame.ok);

    // Log transaction hash
    printf("Game created Transaction hash: 0x");
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", resCreateGame.ok.data[i]);
    }
    printf("\n");

    return 0;


    // signing key
    FieldElement signing_key = {};
    hex_to_bytes(player_signing_key, &signing_key);

    // provider
    ResultProvider resProvider = provider_new(rpc_url);
    if (resProvider.tag == ErrProvider)
    {
        printf("Failed to create provider: %s\n", resProvider.err.message);
        return 1;
    }
    struct Provider *provider = resProvider.ok;

    // account
    ResultAccount resAccount = account_new(provider, signing_key, player_address);
    if (resAccount.tag == ErrAccount)
    {
        printf("Failed to create account: %s\n", resAccount.err.message);
        return 1;
    }
    struct Account *master_account = resAccount.ok;

    FieldElement master_address = account_address(master_account);
    printf("Master account: 0x");
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", master_address.data[i]);
    }
    printf("\n");

    FieldElement burner_signer = signing_key_new();
    ResultAccount resBurner = account_deploy_burner(provider, master_account, burner_signer);
    if (resBurner.tag == ErrAccount)
    {
        printf("Failed to create burner: %s\n", resBurner.err.message);
        return 1;
    }

    struct Account *burner = resBurner.ok;

    printf("Burner account: 0x");
    FieldElement burner_address = account_address(burner);
    for (size_t i = 0; i < 32; i++)
    {
        printf("%02x", burner_address.data[i]);
    }
    printf("\n");

    Query query = {};
    query.limit = 100;
    query.clause.tag = NoneClause;
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

    ResultSubscription resEntityUpdate = client_on_entity_state_update(client, (void*)0, 0, &on_entity_state_update);
    if (resEntityUpdate.tag == ErrSubscription)
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

    // move left felt(0x01)
    hex_to_bytes("0x01", &move.calldata.data[0]);

    BlockId block_id = {
        .tag = BlockTag_,
        .block_tag = Pending,
    };
    account_set_block_id(master_account, block_id);
    ResultFieldElement resSpawn = account_execute_raw(master_account, &spawn, 1);
    if (resSpawn.tag == Errbool)
    {
        printf("Failed to execute call: %s\n", resSpawn.err.message);
        return 1;
    }
    wait_for_transaction(provider, resSpawn.ok);

    printf("Spawned\n");

    ResultFieldElement resMove = account_execute_raw(master_account, &move, 1);
    if (resMove.tag == Errbool)
    {
        printf("Failed to execute call: %s\n", resMove.err.message);
        return 1;
    }
    wait_for_transaction(provider, resMove.ok);

    printf("Moved\n");

    // account_set_block_id(burner, block_id);
    // resSpawn = account_execute_raw(burner, &spawn, 1);
    // if (resSpawn.tag == Errbool)
    // {
    //     printf("Failed to execute call: %s\n", resSpawn.err.message);
    //     return 1;
    // }
    // wait_for_transaction(provider, resSpawn.ok);

    // printf("Spawned burner\n");

    // resMove = account_execute_raw(burner, &move, 1);
    // if (resMove.tag == Errbool)
    // {
    //     printf("Failed to execute call: %s\n", resMove.err.message);
    //     return 1;
    // }
    // wait_for_transaction(provider, resMove.ok);

    // printf("Moved burner\n");

    while (1)
    {
    }

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