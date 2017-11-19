extern crate ws;

use std::sync::{Arc, Mutex};
use buffer::{Buffer};

use ws::{Handler, Sender, Message, Result, CloseCode, Handshake, Error};

pub struct Server {
    pub conn: Sender,
    pub buffer: Arc<Mutex<Buffer>>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(ip) = try!(shake.remote_addr()) {
            eprintln!("Client connected from {}", ip);
        } else {
            eprintln!("Client connected from unknown IP");
        }

        let mut shared = self.buffer.lock().unwrap();

        for line in shared.get_buffer() {
            let message = Message::Text(line.clone());
            self.conn.send(message).unwrap()
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        eprintln!("Client sent message '{}'", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        eprintln!("Client closed connection ({:?}) {}", code, reason)
    }

    fn on_error(&mut self, _err: Error) {
        eprintln!("Client closed connection")
    }
}
