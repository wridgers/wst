extern crate ws;

mod buffer;
mod client;
mod server;

use std::env;
use std::io::BufRead;
use std::io;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

use ws::{connect, WebSocket};

use buffer::{Buffer};
use client::{Client};
use server::{Server};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn usage_general() {
    println!("Version: {}
Usage: wst MODE [arguments]

Modes:
  server HOST PORT  - a server that broadcasts stdin to connected clients
  client URL        - a client that prints messages to stdout (default)
  version           - show version
  help              - show help", VERSION);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage_general();
        exit(0);
    }

    let arg = &args[1];

    match &arg[..] {
        "help" => {
            usage_general();
        },

        "version" => {
            println!("{}", VERSION)
        }

        "server" => {
            if args.len() < 4 {
                usage_general();
                exit(0);
            }

            let buffer = Arc::new(Mutex::new(Buffer::new(1, 10)));

            let listen = format!("{}:{}", &args[2], &args[3]);
            let server = WebSocket::new(|conn| {
                let buffer_clone = buffer.clone();

                Server {
                    conn: conn,
                    buffer: buffer_clone,
                }
            } ).unwrap();

            let broadcaster = server.broadcaster();

            let buffer_clone = buffer.clone();
            let input = thread::spawn(move || {
                let stdin = io::stdin();

                for line in stdin.lock().lines() {
                    let line = line.unwrap();

                    let mut shared = buffer_clone.lock().unwrap();
                    shared.add_line(line.clone());

                    println!("{}", line);
                    broadcaster.send(line).unwrap();
                }
            });

            println!("Starting server on {}", listen);
            server.listen(listen).unwrap();
            input.join().unwrap()
        },

        "client" => {
            if args.len() < 3 {
                usage_general();
                exit(0);
            }

            let url = &args[2];
            connect(url.to_string(), |_| Client {}).unwrap()
        },

        _ => {
            connect(arg.to_string(), |_| Client {}).unwrap()
        }
    }
}
