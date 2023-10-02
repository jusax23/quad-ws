use std::ffi::CString;

pub type WsChannnel = i32;

extern "C" {
    fn ws_open(ptr: *const i8, len: u32) -> WsChannnel;
    fn ws_revive(id: WsChannnel) -> bool;
    fn ws_write(id: WsChannnel, ptr: *const u8, len: u32) -> bool;
    fn ws_read(id: WsChannnel, ptr: *const u8, len: u32);
    fn ws_available(id: WsChannnel) -> i32;
    fn ws_state(id: WsChannnel) -> i32;
    fn ws_close(id: WsChannnel);
}

pub fn ws_open_rust(url: String) -> Option<WsChannnel> {
    let url = CString::new(url).unwrap();
    let socket_id = unsafe { ws_open(url.as_ptr(), url.as_bytes().len() as u32) };
    if socket_id < 0 {
        None
    } else {
        Some(socket_id)
    }
}

pub fn ws_revive_rust(socket: &mut WsChannnel) -> bool {
    return unsafe { ws_revive(*socket) };
}

pub fn ws_write_rust(socket: &mut WsChannnel, data: Vec<u8>) -> bool {
    let buf = data.as_slice();
    let succ = unsafe { ws_write(*socket, buf.as_ptr(), buf.len() as u32) };
    succ
}

pub fn ws_read_rust(socket: &mut WsChannnel) -> Option<Vec<u8>> {
    let available = unsafe { ws_available(*socket) };
    if available < 0 {
        return None;
    }
    let buffer = vec![0; available as usize];
    unsafe { ws_read(*socket, buffer.as_ptr(), available as u32) };
    return Some(buffer);
}

pub fn ws_close_rust(socket: &mut WsChannnel) {
    unsafe { ws_close(*socket) };
}

pub fn ws_state_rust(socket: &mut WsChannnel) -> i32 {
    let state = unsafe { ws_state(*socket) };
    return state;
}
