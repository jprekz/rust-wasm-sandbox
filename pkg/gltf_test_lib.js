import { __cargo_web_snippet_8c32019649bb581b1b742eeedfc410e2bedd56a6 } from './snippets/stdweb-bb142200b065bd55/inline139.js';
import { __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0 } from './snippets/stdweb-bb142200b065bd55/inline370.js';
import { wasm_bindgen_initialize } from './snippets/stdweb-bb142200b065bd55/inline45.js';
import { __cargo_web_snippet_5c3091ae7fa9c42123eec37f64de99a5808e7ef2 } from './snippets/stdweb-bb142200b065bd55/inline460.js';
import { __cargo_web_snippet_5c57e16ebd22655f976d87fae8039e282d7bab59 } from './snippets/stdweb-bb142200b065bd55/inline464.js';
import { __cargo_web_snippet_ecd8f83530fd9b57edbdc4822b4ea5b373e3a927 } from './snippets/stdweb-bb142200b065bd55/inline472.js';
import { __cargo_web_snippet_9a7eca60ef721cc6257c116658f2fcdd49070cd1 } from './snippets/stdweb-bb142200b065bd55/inline566.js';
import { __cargo_web_snippet_7da4da5c1d6778b3677e0a3133c1c2613594e3f4 } from './snippets/stdweb-bb142200b065bd55/inline567.js';
import { __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723 } from './snippets/stdweb-bb142200b065bd55/inline598.js';
import { __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6 } from './snippets/stdweb-bb142200b065bd55/inline599.js';
import { __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf } from './snippets/stdweb-bb142200b065bd55/inline600.js';
import { __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63 } from './snippets/stdweb-bb142200b065bd55/inline601.js';
import * as __wbg_star0 from './snippets/gltf_test-bca6e5be81440404/inline0.js';

let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he6c30fd142ff59f0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_30(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_33(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_36(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he25ff6182870bd26(arg0, arg1);
}

function __wbg_adapter_39(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_42(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_45(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_48(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11f70b97447f9cfb(arg0, arg1, addHeapObject(arg2));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_51(arg0, arg1, arg2) {
    var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5359d0080139188e(arg0, arg1, arg2);
    return ret;
}

function __wbg_adapter_54(arg0, arg1, arg2, arg3) {
    wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb6e33b5d3da881f1(arg0, arg1, arg2, arg3);
}

/**
*/
export function start() {
    wasm.start();
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cargowebsnippet80d6d56760c65e49b7be8b6b01c1ea861b046bf0_5a8953894b8affd6 = function(arg0, arg1) {
        __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_wasmbindgeninitialize_c1c4df6b494511ad = function(arg0, arg1, arg2, arg3) {
        var ret = wasm_bindgen_initialize(takeObject(arg0), takeObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cargowebsnippet8c32019649bb581b1b742eeedfc410e2bedd56a6_fe72322db9f33c63 = function(arg0, arg1, arg2) {
        __cargo_web_snippet_8c32019649bb581b1b742eeedfc410e2bedd56a6(takeObject(arg0), arg1, arg2);
    };
    imports.wbg.__wbg_cargowebsnippetecd8f83530fd9b57edbdc4822b4ea5b373e3a927_4b05cce5d27da5ee = function(arg0, arg1) {
        var ret = __cargo_web_snippet_ecd8f83530fd9b57edbdc4822b4ea5b373e3a927(takeObject(arg0), arg1);
        return ret;
    };
    imports.wbg.__wbg_cargowebsnippet5c3091ae7fa9c42123eec37f64de99a5808e7ef2_fc0bd84666f3fba5 = function(arg0, arg1) {
        var ret = __cargo_web_snippet_5c3091ae7fa9c42123eec37f64de99a5808e7ef2(takeObject(arg0), arg1);
        return ret;
    };
    imports.wbg.__wbg_cargowebsnippet5c57e16ebd22655f976d87fae8039e282d7bab59_ad29da5ae967b2d2 = function(arg0, arg1) {
        __cargo_web_snippet_5c57e16ebd22655f976d87fae8039e282d7bab59(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_cargowebsnippet9a7eca60ef721cc6257c116658f2fcdd49070cd1_da979f7a7a4ebfc0 = function(arg0, arg1) {
        __cargo_web_snippet_9a7eca60ef721cc6257c116658f2fcdd49070cd1(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_cargowebsnippet7da4da5c1d6778b3677e0a3133c1c2613594e3f4_df29dfefa4c72388 = function(arg0, arg1) {
        var ret = __cargo_web_snippet_7da4da5c1d6778b3677e0a3133c1c2613594e3f4(takeObject(arg0), arg1);
        return ret;
    };
    imports.wbg.__wbg_cargowebsnippet1c30acb32a1994a07c75e804ae9855b43f191d63_6d353463ef525961 = function(arg0) {
        __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetdc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf_ce5c721cab10d020 = function(arg0) {
        __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet97495987af1720d8a9a923fa4683a7b683e3acd6_a438202dc16f44c0 = function(arg0, arg1, arg2) {
        __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6(takeObject(arg0), arg1, arg2);
    };
    imports.wbg.__wbg_cargowebsnippet72fc447820458c720c68d0d8e078ede631edd723_ece3da0a4474dbeb = function(arg0, arg1, arg2, arg3) {
        __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723(takeObject(arg0), arg1, arg2, arg3);
    };
    imports.wbg.__wbg_bindVertexArray_8bb02f8645a29e05 = function(arg0, arg1) {
        getObject(arg0).bindVertexArray(getObject(arg1));
    };
    imports.wbg.__wbg_bufferData_12efaaa2e1cd74e4 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferSubData_9367b8c9fdec36a1 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
    };
    imports.wbg.__wbg_createVertexArray_fd08eb7c8f8e86a3 = function(arg0) {
        var ret = getObject(arg0).createVertexArray();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteVertexArray_b966baf643cbbf5b = function(arg0, arg1) {
        getObject(arg0).deleteVertexArray(getObject(arg1));
    };
    imports.wbg.__wbg_texImage2D_917b0bb22a5467b7 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
    });
    imports.wbg.__wbg_uniformMatrix2fv_309b3ff9c01def66 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_4c40c5b08807ce96 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_7a754e2c7af71be4 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_activeTexture_9d96cecdacbe1a7d = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_1924aa4a49a31418 = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindAttribLocation_3cf80d1673eb27d7 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_bindBuffer_6a7df3ea760a2c83 = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_a03a7320443c8a4d = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_clear_256f95c85e2d5b47 = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_5941bfbf220e0165 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_compileShader_18c92b61889a02b6 = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_7fadf474857a2122 = function(arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_8d6f13ab051f686a = function(arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_bc89b940e81883dd = function(arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_e172faa9d6a303c1 = function(arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteBuffer_e06fcb6201291d2e = function(arg0, arg1) {
        getObject(arg0).deleteBuffer(getObject(arg1));
    };
    imports.wbg.__wbg_deleteProgram_c33ed33d83373050 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_80c56945efc9e910 = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_deleteTexture_71c09d0186504319 = function(arg0, arg1) {
        getObject(arg0).deleteTexture(getObject(arg1));
    };
    imports.wbg.__wbg_depthFunc_b6ca07790535145e = function(arg0, arg1) {
        getObject(arg0).depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_depthMask_aa1c19372a664ed8 = function(arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_depthRange_c4c23f8abc72bc40 = function(arg0, arg1, arg2) {
        getObject(arg0).depthRange(arg1, arg2);
    };
    imports.wbg.__wbg_disable_edb7c38f0be19a38 = function(arg0, arg1) {
        getObject(arg0).disable(arg1 >>> 0);
    };
    imports.wbg.__wbg_disableVertexAttribArray_30ea15bb3b3341be = function(arg0, arg1) {
        getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_drawElements_0376b48b08ac34a7 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_enable_28a715ea384ce803 = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_fafa57fbcd454495 = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_generateMipmap_e4fa2d997c6d5f54 = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getProgramInfoLog_221be6701c636176 = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_d2854e9210e85494 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_1071a8467544f43b = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_f942fc2044b16ba0 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_d6e4f5bee8a84579 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_a5fd2d3a29f244c0 = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_shaderSource_1804c02eec34a9c2 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameteri_f3be7a9c7fc03dac = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1f_4718a6fd4ea1bc6c = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform1i_a34df477d48c37e2 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform2f_3bcd4901604aedef = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform2i_a25cefb612c3dfed = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2i(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform3f_f6756fdfc5833abe = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform3i_ee13756b7d027109 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3i(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform4f_1a2d24f9cb947d76 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_uniform4i_ab9bc1a73c5284ce = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4i(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_useProgram_9523fdac78894a60 = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_88010123ef756633 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_30b14839e31d0b61 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_instanceof_Window_adf3196bdc02b386 = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_6cc8d0b87c0a99b9 = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_innerWidth_60241abd729ed26f = handleError(function(arg0) {
        var ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_innerHeight_2f860a67225f1fbd = handleError(function(arg0) {
        var ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_devicePixelRatio_599d41a9267fa1ca = function(arg0) {
        var ret = getObject(arg0).devicePixelRatio;
        return ret;
    };
    imports.wbg.__wbg_cancelAnimationFrame_7f3ba4191e67c86b = handleError(function(arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    });
    imports.wbg.__wbg_matchMedia_443fe61cbc261085 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).matchMedia(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_requestAnimationFrame_89935c9d6ac25d2f = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    });
    imports.wbg.__wbg_clearTimeout_dd3fc8919742efa0 = function(arg0, arg1) {
        getObject(arg0).clearTimeout(arg1);
    };
    imports.wbg.__wbg_fetch_91f098921cc7cca8 = function(arg0, arg1) {
        var ret = getObject(arg0).fetch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_eaf00c9296a6ab88 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    });
    imports.wbg.__wbg_setProperty_42eabadfcd7d6199 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_addEventListener_9e7b0c3f65ebc0d7 = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    });
    imports.wbg.__wbg_offsetX_204ab4b52fb9d668 = function(arg0) {
        var ret = getObject(arg0).offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_3d8860ff1285d58d = function(arg0) {
        var ret = getObject(arg0).offsetY;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_6ff5d3368ab5c157 = function(arg0) {
        var ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_3ed819c2157c6170 = function(arg0) {
        var ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_70d44f4345c386d7 = function(arg0) {
        var ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_bd8efb2d7ff639ad = function(arg0) {
        var ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_button_13536d578538c005 = function(arg0) {
        var ret = getObject(arg0).button;
        return ret;
    };
    imports.wbg.__wbg_bindVertexArrayOES_e8d5f6f1ec4f0ecc = function(arg0, arg1) {
        getObject(arg0).bindVertexArrayOES(getObject(arg1));
    };
    imports.wbg.__wbg_createVertexArrayOES_e430ded3ba919d50 = function(arg0) {
        var ret = getObject(arg0).createVertexArrayOES();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteVertexArrayOES_6b57bb46ff0c7688 = function(arg0, arg1) {
        getObject(arg0).deleteVertexArrayOES(getObject(arg1));
    };
    imports.wbg.__wbg_deltaX_ee242e8414135d41 = function(arg0) {
        var ret = getObject(arg0).deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_35bf8632b9f25820 = function(arg0) {
        var ret = getObject(arg0).deltaY;
        return ret;
    };
    imports.wbg.__wbg_deltaMode_1fd222964cdbb5af = function(arg0) {
        var ret = getObject(arg0).deltaMode;
        return ret;
    };
    imports.wbg.__wbg_pointerId_e7b9f2d5782623b5 = function(arg0) {
        var ret = getObject(arg0).pointerId;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Response_328c03967a8e8902 = function(arg0) {
        var ret = getObject(arg0) instanceof Response;
        return ret;
    };
    imports.wbg.__wbg_arrayBuffer_dc33ab7b8cdf0d63 = handleError(function(arg0) {
        var ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_matches_9d45aa12636791b2 = function(arg0) {
        var ret = getObject(arg0).matches;
        return ret;
    };
    imports.wbg.__wbg_now_49847177a6d1d57e = function(arg0) {
        var ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_newwithstrandinit_d1de1bfcd175e38a = handleError(function(arg0, arg1, arg2) {
        var ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_settitle_165e2beb3fd737a8 = function(arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_body_8c888fe47d81765f = function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_head_5b8d72423fc1599b = function(arg0) {
        var ret = getObject(arg0).head;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_fullscreenElement_449980d04fa17948 = function(arg0) {
        var ret = getObject(arg0).fullscreenElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_5bdf88a5af9f17c5 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_requestFullscreen_7c9e8be46f97059a = handleError(function(arg0) {
        getObject(arg0).requestFullscreen();
    });
    imports.wbg.__wbg_setAttribute_727bdb9763037624 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_remove_cc7ef8dcae8cd130 = function(arg0) {
        getObject(arg0).remove();
    };
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_a37cc8c6016098e4 = function(arg0) {
        var ret = getObject(arg0) instanceof WebGLRenderingContext;
        return ret;
    };
    imports.wbg.__wbg_bufferData_9e8c2779e1cc79b5 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferSubData_a9b3397bba440942 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
    };
    imports.wbg.__wbg_texImage2D_8d677a54ab75452c = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
    });
    imports.wbg.__wbg_uniformMatrix2fv_5891b91c273f037a = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_a197f321b6f12a6c = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_8f6f3c8389e3b449 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_activeTexture_7246ae8c464868b4 = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_d213e7ecd3432f4a = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindAttribLocation_20393eae552494a7 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_bindBuffer_f0ba4bbfd5b08434 = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_c00656e6f0530ee7 = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_clear_c9cc14c37d12a838 = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_73695d8d401f87e6 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_compileShader_961db910485f4a76 = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_4deb008968921e7f = function(arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_b502951c403f671a = function(arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_7bd4296ba9c32133 = function(arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_e0437703d5b41f24 = function(arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteBuffer_decedf58d645291f = function(arg0, arg1) {
        getObject(arg0).deleteBuffer(getObject(arg1));
    };
    imports.wbg.__wbg_deleteProgram_bc45913c88af2629 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_b4d94258df702172 = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_deleteTexture_69cdd9674b9207ce = function(arg0, arg1) {
        getObject(arg0).deleteTexture(getObject(arg1));
    };
    imports.wbg.__wbg_depthFunc_e7ae732468ee0ba0 = function(arg0, arg1) {
        getObject(arg0).depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_depthMask_07065072e66aca40 = function(arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_depthRange_b36243ddda50c078 = function(arg0, arg1, arg2) {
        getObject(arg0).depthRange(arg1, arg2);
    };
    imports.wbg.__wbg_disable_6d4d32f05b00518e = function(arg0, arg1) {
        getObject(arg0).disable(arg1 >>> 0);
    };
    imports.wbg.__wbg_disableVertexAttribArray_23f37b3aae712b33 = function(arg0, arg1) {
        getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_drawElements_b22db7173101346e = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_enable_700dbd1724c67920 = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_4b6614b028d442ff = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_generateMipmap_4221d0bdf6dd9d9e = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getExtension_d4422753d7c5b70a = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_getProgramInfoLog_a84afc629d343c75 = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_327111ebb2bca7fb = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_a9529ee3f2ebd3e0 = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_d7853b2d4822ad9f = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_55700686ebe625a9 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_7c29f15a5150d174 = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_shaderSource_bf6be2cc97a14fc1 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameteri_c9ce5bb9e350c6cd = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1f_aef9f13b531f3269 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform1i_bbbce4278738d73e = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform2f_aaa7fe970579cfd3 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform2i_3e5fe50aae1281e2 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2i(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform3f_ced8926a298355a4 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform3i_f00523bf726b8562 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3i(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform4f_47bfe7de98565851 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_uniform4i_518533c7512c575f = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4i(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_useProgram_51f7808f5955c03a = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_76ddec1ed8425967 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_dd0dedc488a8dba4 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_style_9a41d46c005f7596 = function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_focus_5f74cb245be46131 = handleError(function(arg0) {
        getObject(arg0).focus();
    });
    imports.wbg.__wbg_charCode_96fab95517f6f4f4 = function(arg0) {
        var ret = getObject(arg0).charCode;
        return ret;
    };
    imports.wbg.__wbg_keyCode_689d196ab65a93d7 = function(arg0) {
        var ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_altKey_1b58e09f218a0f4b = function(arg0) {
        var ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_f080ec163dcc2703 = function(arg0) {
        var ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_d11f615955404512 = function(arg0) {
        var ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_9bc40bb1d5972ef2 = function(arg0) {
        var ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_key_590d4d2a765d1b58 = function(arg0, arg1) {
        var ret = getObject(arg1).key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_code_c3b28f37b4149e68 = function(arg0, arg1) {
        var ret = getObject(arg1).code;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_cancelBubble_ba020b9735aa529e = function(arg0) {
        var ret = getObject(arg0).cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_stopPropagation_a47dd3b6ffe6b400 = function(arg0) {
        getObject(arg0).stopPropagation();
    };
    imports.wbg.__wbg_width_a22f9855caa54b53 = function(arg0) {
        var ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_setwidth_5f26a8ba9dbfa0d0 = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_height_9a404a6b3c61c7ef = function(arg0) {
        var ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_setheight_70f62727aa9383c2 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_e7747f5b022c18e9 = handleError(function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_addListener_4c9824b666156827 = handleError(function(arg0, arg1) {
        getObject(arg0).addListener(getObject(arg1));
    });
    imports.wbg.__wbg_appendChild_77215fd672b162c5 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_get_0e3f2950cdf758ae = handleError(function(arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_call_8e95613cc6524977 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_newnoargs_f3b8a801d5d4b079 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_e4679013d435841e = function() {
        var ret = new Map();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_155dce297caef4bd = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fromEntries_e51e19fc1d1624ee = handleError(function(arg0) {
        var ret = Object.fromEntries(getObject(arg0));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_is_333329c4a02916de = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_3e06d4f36713e4cb = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_3b7ac098cfda2fa5 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_07b2f89e82ceb76d = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_window_ba85d88572adc0dc = handleError(function() {
        var ret = window.window;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_globalThis_b9277fc37e201fe5 = handleError(function() {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_global_e16303fe83e1d57f = handleError(function() {
        var ret = global.global;
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_buffer_49131c283a06686f = function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_c0f38401daad5a22 = function(arg0, arg1, arg2) {
        var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_2b13641a9d906653 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_9b295d24cf1d706f = function(arg0) {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_3bb960a9975f3cd2 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_function_table = function() {
        var ret = wasm.__wbindgen_export_2;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1252 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 296, __wbg_adapter_24);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1320 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_27);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1322 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_30);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1324 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_33);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1326 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1328 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_39);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1330 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_42);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1332 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_45);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1334 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 325, __wbg_adapter_48);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1640 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 434, __wbg_adapter_51);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1642 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 434, __wbg_adapter_54);
        return addHeapObject(ret);
    };
    imports['./snippets/gltf_test-bca6e5be81440404/inline0.js'] = __wbg_star0;

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

