const { ToriiClient } = require('./pkg/dojo_c');

// Provide a global WebSocket implementation, which is expected by the WASM code built for browsers.
global.WebSocket = WebSocket;
// Workaround for Node.js as it currently lacks support for Web Workers by pretending there is
// a WorkerGlobalScope object available which is checked within the libp2p's websocket-websys transport.
global.WorkerGlobalScope = global;


require('./pkg');

async function main() {
    const client = await (new ToriiClient({
        toriiUrl: 'http://127.0.0.1:8080',
        relayUrl: '/ip4/127.0.0.1/tcp/9092/ws',
        worldAddress: '0x0'
    }));

    console.log(await client.getAllEntities(10))
}

main().catch(console.error);