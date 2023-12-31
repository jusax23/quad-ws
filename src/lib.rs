#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod pc;
#[cfg(not(target_arch = "wasm32"))]
use pc::*;

#[repr(i32)]
pub enum QuadWsState {
    WsDisconnected = 0,
    WsConnected = 1,
    WsClosed = 2,
}
pub struct QuadWs {
    channel: WsChannnel,
}
impl QuadWs {
    pub fn new(url: String) -> Option<Self> {
        let conn = ws_open_rust(url);
        if conn.is_none() {
            return None;
        }
        Some(Self {
            channel: conn.unwrap(),
        })
    }
    pub fn write(&mut self, data: Vec<u8>) -> bool {
        ws_write_rust(&mut self.channel, data)
    }
    pub fn close(&mut self) {
        ws_close_rust(&mut self.channel)
    }
    pub fn read(&mut self) -> Option<Vec<u8>> {
        ws_read_rust(&mut self.channel)
    }
    pub fn connected(&mut self) -> bool {
        if let QuadWsState::WsConnected = self.state() {
            return true;
        }
        false
    }
    pub fn revive(&mut self) {
        ws_revive_rust(&mut self.channel);
    }
    pub fn state(&mut self) -> QuadWsState {
        let state = ws_state_rust(&mut self.channel);
        if state < 0 || state > 2 {
            return QuadWsState::WsClosed;
        }
        unsafe { ::std::mem::transmute(state) }
    }
}
