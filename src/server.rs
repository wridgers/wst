extern crate ws;
extern crate time;

use std::sync::{Arc, Mutex};
use std::str::from_utf8;

use buffer::{Buffer};

use ws::{Handler, Sender, Message, Result, CloseCode, Handshake, Error, ErrorKind, Frame, OpCode};
use ws::util::{Token, Timeout};

const PING: Token = Token(1);

pub struct Server {
    pub conn: Sender,
    pub buffer: Arc<Mutex<Buffer>>,
    pub ping_timeout: Option<Timeout>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(ip) = try!(shake.remote_addr()) {
            eprintln!("Client connected from {}", ip);
        } else {
            eprintln!("Client connected from unknown IP");
        }

        try!(self.conn.timeout(5_000, PING));

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
        eprintln!("Client closed connection ({:?}) {}", code, reason);

        if let Some(t) = self.ping_timeout.take() {
            self.conn.cancel(t).unwrap();
        }
    }

    fn on_error(&mut self, _err: Error) {
        eprintln!("Client closed connection")
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            PING => {
                try!(self.conn.ping(time::precise_time_ns().to_string().into()));
                self.ping_timeout.take();
                self.conn.timeout(5_000, PING)
            }
            _ => Err(Error::new(ErrorKind::Internal, "Invalid timeout token encountered!")),
        }
    }

    fn on_new_timeout(&mut self, _event: Token, timeout: Timeout) -> Result<()> {
        if let Some(t) = self.ping_timeout.take() {
            try!(self.conn.cancel(t))
        }

        self.ping_timeout = Some(timeout);

        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        if frame.opcode() == OpCode::Pong {
            if let Ok(pong) = try!(from_utf8(frame.payload())).parse::<u64>() {
                let now = time::precise_time_ns();

                eprintln!("Received pong. RTT is {:.3}ms.", (now - pong) as f64 / 1_000_000f64);
            } else {
                eprintln!("Received bad pong.");
            }
        }

        DefaultHandler.on_frame(frame)
    }
}

struct DefaultHandler;
impl Handler for DefaultHandler {}
