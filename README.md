# WebSocket Tool

A utility for working with websockets.

[![Crates.io](https://img.shields.io/crates/v/wst.svg)](https://crates.io/crates/wst) [![license](https://img.shields.io/github/license/wridgers/wst.svg)]()

## Install

    cargo install wst

## Usage

`wst` can act as both a client and a server.

    $ wst
    Version: 0.2.0
    Usage: wst MODE [arguments]

    Modes:
      server HOST PORT      A server that broadcasts stdin to connected clients
      client URL            A client that prints messages to stdout (default)
      version               Show version
      help                  Show help

    Run 'wst MODE help' for more information on a mode.

As a **client** `wst` will print received messages as a line to stdout. The `client` mode argument is optional.

    wst client ws://localhost:9000
    wst ws://localhost:9000

As a **server** `wst` will transmit lines received on stdin to connected clients.

    ( while :; do date && sleep 1; done) | wst server localhost 9000

## Licence

MIT
