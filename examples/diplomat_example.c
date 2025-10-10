/**
 * Example demonstrating the new Diplomat-generated C bindings for dojo.c
 * 
 * Compile with:
 *   clang examples/diplomat_example.c -I bindings/c -L target/debug -ldojo_c -o diplomat_example
 * 
 * Run with:
 *   LD_LIBRARY_PATH=target/debug ./diplomat_example
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../bindings/c/SigningKey.h"
#include "../bindings/c/FieldElement.h"
#include "../bindings/c/ToriiClient.h"
#include "../bindings/c/Provider.h"
#include "../bindings/c/Account.h"
#include "../bindings/c/CallList.h"
#include "../bindings/c/DojoError.h"

void print_write(DiplomatWrite* write) {
    if (write->len > 0) {
        char* str = malloc(write->len + 1);
        memcpy(str, write->buf, write->len);
        str[write->len] = '\0';
        printf("%s\n", str);
        free(str);
    }
}

int main() {
    printf("=== Diplomat Dojo C Bindings Example ===\n\n");

    // 1. Create a random signing key
    printf("1. Creating random signing key...\n");
    SigningKey* signer = SigningKey_from_random();
    
    char secret_buffer[256] = {0};
    DiplomatWrite secret_write = {secret_buffer, 256, 0, false};
    SigningKey_secret_scalar(signer, &secret_write);
    printf("   Secret scalar: ");
    print_write(&secret_write);
    
    // 2. Create a field element from hex
    printf("\n2. Creating field element from hex...\n");
    const char* hex_value = "0x1234567890abcdef";
    DiplomatStringView hex_view = {hex_value, strlen(hex_value)};
    
    FieldElement_new_from_hex_result felt_result = FieldElement_new_from_hex(hex_view);
    if (felt_result.is_ok) {
        FieldElement* felt = felt_result.ok;
        char felt_buffer[256] = {0};
        DiplomatWrite felt_write = {felt_buffer, 256, 0, false};
        FieldElement_to_hex(felt, &felt_write);
        printf("   Field element: ");
        print_write(&felt_write);
        FieldElement_destroy(felt);
    } else {
        printf("   Error creating field element\n");
        DojoError_destroy(felt_result.err);
    }
    
    // 3. Create a Torii client
    printf("\n3. Creating Torii client...\n");
    const char* torii_url = "http://localhost:8080";
    DiplomatStringView url_view = {torii_url, strlen(torii_url)};
    
    ToriiClient_new_result client_result = ToriiClient_new(url_view);
    if (client_result.is_ok) {
        ToriiClient* client = client_result.ok;
        printf("   Client created successfully\n");
        
        char info_buffer[256] = {0};
        DiplomatWrite info_write = {info_buffer, 256, 0, false};
        ToriiClient_info_result info_result = ToriiClient_info(client, &info_write);
        if (info_result.is_ok) {
            printf("   Info: ");
            print_write(&info_write);
        }
        
        ToriiClient_destroy(client);
    } else {
        printf("   Error creating client: ");
        char err_buffer[512] = {0};
        DiplomatWrite err_write = {err_buffer, 512, 0, false};
        DojoError_message(client_result.err, &err_write);
        print_write(&err_write);
        DojoError_destroy(client_result.err);
    }
    
    // 4. Create a provider
    printf("\n4. Creating JSON-RPC provider...\n");
    const char* rpc_url = "https://api.cartridge.gg/x/starknet/mainnet";
    DiplomatStringView rpc_view = {rpc_url, strlen(rpc_url)};
    
    Provider_new_result provider_result = Provider_new(rpc_view);
    if (provider_result.is_ok) {
        Provider* provider = provider_result.ok;
        printf("   Provider created successfully\n");
        
        char chain_buffer[256] = {0};
        DiplomatWrite chain_write = {chain_buffer, 256, 0, false};
        Provider_chain_id_result chain_result = Provider_chain_id(provider, &chain_write);
        if (chain_result.is_ok) {
            printf("   Chain ID: ");
            print_write(&chain_write);
        } else {
            printf("   Could not fetch chain ID (network may be unavailable)\n");
            DojoError_destroy(chain_result.err);
        }
        
        Provider_destroy(provider);
    } else {
        printf("   Error creating provider\n");
        DojoError_destroy(provider_result.err);
    }
    
    // 5. Sign a message
    printf("\n5. Signing a message...\n");
    const char* message_hex = "0xdeadbeef";
    DiplomatStringView message_view = {message_hex, strlen(message_hex)};
    
    FieldElement_new_from_hex_result msg_felt_result = FieldElement_new_from_hex(message_view);
    if (msg_felt_result.is_ok) {
        FieldElement* msg_felt = msg_felt_result.ok;
        SigningKey_sign_result sig_result = SigningKey_sign(signer, msg_felt);
        if (sig_result.is_ok) {
            Signature* sig = sig_result.ok;
            printf("   Message signed successfully\n");
            
            char r_buffer[256] = {0};
            DiplomatWrite r_write = {r_buffer, 256, 0, false};
            Signature_r(sig, &r_write);
            printf("   Signature R: ");
            print_write(&r_write);
            
            char s_buffer[256] = {0};
            DiplomatWrite s_write = {s_buffer, 256, 0, false};
            Signature_s(sig, &s_write);
            printf("   Signature S: ");
            print_write(&s_write);
            
            Signature_destroy(sig);
        } else {
            printf("   Error signing message\n");
            DojoError_destroy(sig_result.err);
        }
        FieldElement_destroy(msg_felt);
    }
    
    // Cleanup
    SigningKey_destroy(signer);
    
    printf("\n=== Example completed ===\n");
    return 0;
}

