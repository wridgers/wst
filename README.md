# WebSocket Tool

A utility for working with websockets.

[![Crates.io](https://img.shields.io/crates/v/wst.svg)](https://crates.io/crates/wst) [![license](https://img.shields.io/github/license/wridgers/wst.svg)]()

## Install

    cargo install wst

## Usage

`wst` can act as both a client and a server.

    $ wst
    Usage: wst MODE

    Modes:
      server HOST PORT  - a server that broadcasts stdin to connected clients
      client URL        - a client that prints messages to stdout (default)

As a **server** `wst` will transmit lines received on stdin to connected clients.

    ( while :; do date && sleep 1; done) | wst server localhost 1234

As a **client** `wst` will print received messages as a line to stdout. The `client` mode argument is optional.

    wst client ws://localhost:1234
    wst ws://host:port

## Licence

MIT
