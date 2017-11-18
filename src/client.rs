use ws::{Handler, Handshake, Message, Result, CloseCode, Error};

pub struct Client {}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        eprintln!("Client closed connection ({:?}) {}", code, reason)
    }

    fn on_error(&mut self, _err: Error) {
        eprintln!("Client closed connection")
    }
}
