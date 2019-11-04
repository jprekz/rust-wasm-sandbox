const importObject = {
    env: {
        js_log: log,
    }
};

WebAssembly.instantiateStreaming(fetch("index.wasm"), importObject)
    .then(obj => {
        wasm_inst = obj.instance;
        obj.instance.exports.main();
    });

let wasm_inst;

function log(ptr, byte_size) {
    let u8_array = new Uint8Array(wasm_inst.exports.memory.buffer, ptr, byte_size);
    console.log(decoder.decode(u8_array));
}

let decoder = new TextDecoder(); 
