#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod pc;
#[cfg(not(target_arch = "wasm32"))]
use pc::*;

pub struct QuadWs {
    channel: WsChannnel,
}
impl QuadWs {
    pub fn new(url: String) -> Self {
        Self {
            channel: ws_open_rust(url).unwrap(),
        }
    }
    pub fn write(&mut self) -> bool {
        ws_write_rust(&mut self.channel, vec![])
    }
    pub fn close(&mut self) {
        ws_close_rust(&mut self.channel)
    }
    pub fn read(&mut self) -> Option<Vec<u8>>{
        ws_read_rust(&mut self.channel)
    }
}
