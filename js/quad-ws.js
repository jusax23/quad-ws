"use strict";

/**
 * Represents a WebSocket connection and its state.
 * @typedef {Object} WebSocketConnection
 * @property {WebSocket} socket - The WebSocket instance.
 * @property {number} state - The state of the WebSocket connection. Possible values:
 *   - 0: Connection is in the CONNECTING state.
 *   - 1: Connection is in the OPEN state.
 *   - 2: Connection is in the CLOSING state.
 *   - 3: Connection is in the CLOSED state.
 * @property {Array} received - An array to store received data or messages.
 */

/**
 * An instance of WebSocketConnection.
 * @type {{[key: number]: WebSocketConnection}}
 */

let ws = {};
let i = 0;

const WS_NOT_EXISTING = -1;
const WS_CREATING = 0;
const WS_OPEN = 1;
const WS_CLOSED = 2;

function ws_open(ptr, len) {
    let url = UTF8ToString(ptr, len);
    let id = i++;
    let conn;
    try {
        conn = {
            socket: new WebSocket(url),
            state: 0,
            received: []
        };
    } catch (error) {
        return -1;
    }
    
    conn.socket.onopen = (s, e) => {
        conn.state = WS_OPEN;
    };
    conn.socket.onmessage = (s, e) => {
        conn.received.push(new Uint8Array(e.data.arraybuffer()));
    };
    conn.socket.onerror = (s, e) => {
        conn.state = WS_CLOSED;
    };
    conn.socket.onclose = (s, e) => {
        conn.state = WS_CLOSED;
        delete ws[id];
        ws[id] = undefined;
    };
    ws[id] = conn;
    return id;
}

function ws_write(id, ptr, len) {
    let data = new Uint8Array(wasm_memory.buffer, ptr, len);
    /*let data = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        data[i] = data_in[i];
    }*/
    if (ws[id] != null) {
        ws[id].socket.send(data);
        return true;
    }
    return false;
}
function ws_available(id) {
    if (ws[id].received.length == 0) return -1;
    return ws[id].received[0].length;
}

function ws_read(id, ptr, max_length) {
    let file = ws[id].received.pop();
    var dest = new Uint8Array(wasm_memory.buffer, ptr, max_length);
    for (let i = 0; i < file.length && i < max_length; i++) {
        dest[i] = file[i];
    }
}

function ws_state(id) {
    return ws[id]?.state ?? WS_NOT_EXISTING;
}

function ws_close(id) {
    if (ws[id] == null) return;
    ws[id].socket.close();
    ws[id].state = WS_CLOSED;
}

function register_plugin(importObject) {
    importObject.env.ws_open = ws_open;
    importObject.env.ws_write = ws_write;
    importObject.env.ws_read = ws_read;
    importObject.env.ws_available = ws_available;
    importObject.env.ws_state = ws_state;
    importObject.env.ws_close = ws_close;
}

miniquad_add_plugin({ register_plugin, version: "0.0.1", name: "miniquad_websocket" });

/*
var socket = new WebSocket("ws://127.0.0.1:7878");
            socket.onmessage = function (event) {
                var received = document.getElementById("received");
                var br = document.createElement("BR");
                console.log(event.data);
            };
        	
            function send(data) {
            	
                socket.send(data);
            }
*/