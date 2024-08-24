#include <iostream>
#include "dojo.h" // Include dojo.h instead of dojo.hpp

int main()
{
    // Define your URLs and keys
    const std::string torii_url = "http://localhost:8080";
    const std::string rpc_url = "http://localhost:3000";
    const std::string world = "0x01385f25d20a724edc9c7b3bd9636c59af64cbaf9fcd12f33b3af96b2452f295";
    const std::string playerKey = "0x02038e0daba5c3948a6289e91e2a68dfc28e734a281c753933b8bd331e6d3dae";

    // Setup entities
    std::vector<Dojo::Entity> entities = {
        {"Position", {playerKey}}};

    // Create a new Torii client
    auto toriiClient = Dojo::NewToriiClient(torii_url, rpc_url, "/ip4/127.0.0.1/tcp/9090", world, entities, 1);
    if (!toriiClient)
    {
        std::cerr << "Error creating Torii client." << std::endl;
        return 1;
    }

    // Create a burner address
    auto burnerAddress = toriiClient->createBurner();
    if (burnerAddress.error)
    {
        std::cerr << "Error creating burner address: " << burnerAddress.error << std::endl;
        return 1;
    }

    // Query entities
    auto queriedEntities = toriiClient->entities("query");
    if (queriedEntities.error)
    {
        std::cerr << "Error querying entities: " << queriedEntities.error << std::endl;
        return 1;
    }

    // Print entity details
    for (const auto &entity : queriedEntities)
    {
        std::cout << "Entity ID: " << entity.id << ", Name: " << entity.name << std::endl;
    }

    return 0;
}
