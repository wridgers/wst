extern crate ws;

use ws::{Handler, Sender, Message, Result, CloseCode, Handshake, Error};

pub struct Server {
    pub conn: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(ip) = try!(shake.remote_addr()) {
            eprintln!("Client connected from {}", ip);
        } else {
            eprintln!("Client connected from unknown IP");
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
