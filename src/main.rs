extern crate getopts;
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

use getopts::Options;
use ws::{connect, WebSocket};

use buffer::{Buffer};
use client::{Client};
use server::{Server};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn usage_general() {
    println!("Version: {}
Usage: wst MODE [arguments]

Modes:
  server HOST PORT      A server that broadcasts stdin to connected clients
  client URL            A client that prints messages to stdout (default)
  version               Show version
  help                  Show help

Run 'wst MODE help' for more information on a mode.", VERSION);
}

fn usage_server() {
    println!("Usage: wst server [OPTIONS] HOST PORT

Reads from STDIN and broadcasts each line to all clients as a messages

Options:
  -h N, --header N      The first N lines received on STDIN will be repeated to every new client
  -b N, --banner N      The last N sent messages will be repeated to every new client, excluding header messages");
}

fn usage_client() {
    println!("Usage: wst client URL

Connects to a WebSocket and prints each message received to STDOUT");
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
            if args.len() < 4 || (args.len() == 3 && &args[2] == "help") {
                usage_server();
                exit(0);
            }

            let listen_addr = &args[args.len() - 2];
            let listen_port = &args[args.len() - 1];

            let mut server_opts = Options::new();
            server_opts.optopt("b", "buffer", "length of buffer", "LENGTH");
            server_opts.optopt("h", "header", "length of header", "LENGTH");

            let matches = match server_opts.parse(&args[2..]) {
                Ok(m) => { m },
                Err(f) => {
                    eprintln!("Error: {}", f);
                    usage_server();
                    exit(1);
                },
            };

            let header_length = match matches.opt_str("h") {
                Some(x) => {
                    match x.parse::<usize>() {
                        Ok(x) => { x },
                        Err(f) => {
                            eprintln!("Error: {}", f);
                            usage_server();
                            exit(1);
                        },
                    }
                },
                None => { 0 },
            };

            let buffer_length = match matches.opt_str("b") {
                Some(x) => {
                    match x.parse::<usize>() {
                        Ok(x) => { x },
                        Err(f) => {
                            eprintln!("Error: {}", f);
                            usage_server();
                            exit(1);
                        },
                    }
                },
                None => { 0 },
            };

            eprintln!("Header length: {}", header_length);
            eprintln!("Buffer length: {}", buffer_length);

            let buffer = Arc::new(Mutex::new(Buffer::new(header_length, buffer_length)));

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

            let listen = format!("{}:{}", listen_addr, listen_port);
            println!("Starting server on {}", listen);
            server.listen(listen).unwrap();
            input.join().unwrap()
        },

        "client" => {
            if args.len() < 3 || &args[2] == "help" {
                usage_client();
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
