extern crate ws;

mod client;
mod server;

use std::env;
use std::io::BufRead;
use std::io;
use std::process::exit;
use std::thread;

use ws::{connect, WebSocket};

use client::{Client};
use server::{Server};

fn usage_general() {
    println!("Usage: wst MODE

Modes:
  server HOST PORT  - a server that broadcasts stdin to connected clients
  client URL        - a client that prints messages to stdout (default)");
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

        "server" => {
            if args.len() < 4 {
                usage_general();
                exit(0);
            }

            let listen = format!("{}:{}", &args[2], &args[3]);
            let server = WebSocket::new(|conn| { Server { conn: conn } } ).unwrap();
            let broadcaster = server.broadcaster();

            let input = thread::spawn(move || {
                let stdin = io::stdin();

                for line in stdin.lock().lines() {
                    // FIXME: don't copy here. use read_line instead.
                    let r_line = line.unwrap();

                    println!("{}", r_line);
                    broadcaster.send(r_line).unwrap();
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
