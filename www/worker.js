// import {memory} from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

onmessage = (e) => {
    let cells = e.data;
    console.log('worker received:', cells);
    
    const ws = new WebSocket('ws://localhost:8081');

    ws.onopen = () => {
        console.log('worker-ws: WebSocket connected');
    };

    ws.onmessage = (event) => {
        if (event.data instanceof Blob) {
            event.data.arrayBuffer().then(buffer => {
                console.log('worker-ws: received binary data, size:', buffer.byteLength);
                let dv = new DataView(buffer);
                if(dv.getUint8(0) == "G".charCodeAt(0) && dv.getUint8(1) == "O".charCodeAt(0) && dv.getUint8(2) == "L".charCodeAt(0)) {
                    const payload_len = new DataView(buffer).getUint16(3, false);
                    
                    console.log('worker-ws: payload size:', payload_len);
                    
                    // uncommenting this, for some reason makes the webworker not receive messages from the main thread anymore...
                    // const wasmMemory = new Uint8Array(memory.buffer, cells, payload_len);
                    // wasmMemory.set(new Uint8Array(buffer, 5));
                    
                    // Notify main thread that data was "processed"
                    self.postMessage(buffer, [buffer]);
                }
                else {
                    console.warn('worker-ws: unrecognized binary data format');
                }
            })
            .catch(err => {
                console.error('worker-ws: error reading binary data:', err);
            });
            
        } else {
            console.log('worker-ws: received text data:', event.data);
        }
    };

    ws.onerror = (error) => {
        console.error('worker-ws: WebSocket error:', error);
    };

    ws.onclose = () => {
        console.log('worker-ws: WebSocket closed');
    };
};
