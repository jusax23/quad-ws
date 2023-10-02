# quad-ws
WebSocket plugin for miniquad.

Testet on:
- [x] Web: webjs WebSocket  
- [ ] Android  
- [x] Linux: websocket   
- [ ] macOS  
- [ ] Windows
- [ ] iOS

Other Platforms could work, but weren't tested.

## Usage

```rs
let client = QuadWs::new("ws://example.com");
if client.connected() {
    client.write(vec![0, 1, 2, 3, 3]); // connected check is optional / write returns if sending was successfull.
} else {
    client.revive(); // trys to revive the connection.
}

if let Some(buffer) = client.read(){
    //process buffer
}

client.close(); // permanently close the connection.
```

## Attribution
crate websocket is used for none wasm Implementation.