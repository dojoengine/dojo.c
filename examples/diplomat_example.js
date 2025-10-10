/**
 * Example demonstrating the new Diplomat-generated JavaScript bindings for dojo.c
 * 
 * Run with Node.js:
 *   node examples/diplomat_example.js
 * 
 * Or in a browser by importing from the generated bindings
 */

import { SigningKey, FieldElement, ToriiClient, Provider } from '../bindings/js/index.mjs';

async function main() {
    console.log('=== Diplomat Dojo JavaScript Bindings Example ===\n');

    try {
        // 1. Create a random signing key
        console.log('1. Creating random signing key...');
        const signer = SigningKey.fromRandom();
        const secretScalar = signer.secretScalar();
        console.log(`   Secret scalar: ${secretScalar}`);

        // 2. Create a field element from hex
        console.log('\n2. Creating field element from hex...');
        const hexValue = '0x1234567890abcdef';
        try {
            const felt = FieldElement.newFromHex(hexValue);
            const feltHex = felt.toHex();
            console.log(`   Field element: ${feltHex}`);
        } catch (err) {
            console.log(`   Error creating field element: ${err.message()}`);
        }

        // 3. Create a Torii client
        console.log('\n3. Creating Torii client...');
        try {
            const client = ToriiClient.new_('http://localhost:8080');
            console.log('   Client created successfully');
            
            const info = client.info();
            console.log(`   Info: ${info}`);
        } catch (err) {
            console.log(`   Error creating client: ${err.message()}`);
        }

        // 4. Create a provider
        console.log('\n4. Creating JSON-RPC provider...');
        try {
            const provider = Provider.new_('https://api.cartridge.gg/x/starknet/mainnet');
            console.log('   Provider created successfully');
            
            try {
                const chainId = provider.chainId();
                console.log(`   Chain ID: ${chainId}`);
            } catch (err) {
                console.log('   Could not fetch chain ID (network may be unavailable)');
            }
        } catch (err) {
            console.log('   Error creating provider');
        }

        // 5. Sign a message
        console.log('\n5. Signing a message...');
        try {
            const msgFelt = FieldElement.newFromHex('0xdeadbeef');
            const signature = signer.sign(msgFelt);
            console.log('   Message signed successfully');
            console.log(`   Signature R: ${signature.r()}`);
            console.log(`   Signature S: ${signature.s()}`);
        } catch (err) {
            console.log(`   Error signing message: ${err.message()}`);
        }

        // 6. Cryptographic operations
        console.log('\n6. Cryptographic operations...');
        try {
            const verifyingKey = signer.verifyingKey();
            const pubKeyHex = verifyingKey.scalar();
            console.log(`   Public key: ${pubKeyHex}`);
        } catch (err) {
            console.log(`   Error getting verifying key: ${err.message()}`);
        }

        console.log('\n=== Example completed ===');
    } catch (error) {
        console.error('Unexpected error:', error);
    }
}

main().catch(console.error);

