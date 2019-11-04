const imports = {
    log: (ptr, byte_size) => {
        let decoder = new TextDecoder();
        let u8_array = new Uint8Array(wasm_inst.exports.memory.buffer, ptr, byte_size);
        console.log(decoder.decode(u8_array));
    },

    gl_init: () => {
        gl.clearColor(0.6, 0.8, 0.9, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.finish();
    },

    gl_color: (r, g, b) => {
        gl.clearColor(r, g, b, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.finish();
    },

    next_frame: () => {
        let callback = wasm_inst.exports.callback;
        requestAnimationFrame(callback);
    },
}

let canvas = document.getElementById('canvas');
let gl = canvas.getContext('webgl');

let importObject = () => {
    let obj = { env: {} };
    for (let [key, value] of Object.entries(imports)) {
        obj.env["js_" + key] = value;
    }
    return obj;
};

WebAssembly.instantiateStreaming(fetch("index.wasm"), importObject())
    .then(obj => {
        wasm_inst = obj.instance;
        obj.instance.exports.main();
    });

let wasm_inst;

window.addEventListener('beforeunload', (e) => {
    // 16回リロードで発生するwarning対策 for Firefox
    gl.getExtension('WEBGL_lose_context').loseContext();
});
