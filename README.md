# LPR Proxy

## Why

This client/server is for forwarding `lpr` calls from one machine to the other, where each machine has the same sets of files.
The main use is when you want to call `lpr` from a docker but you don't want to bother managing `CUPS` in that docker

## Usage

The client has the same options as `lpr`, with two more `--remote <address>` wich is the address of the remote server, and optionally `--prefix /some/path`, wich will remove from all invocations the said path, having canonicalized the paths first. This is intended to be the path in the docker where a volume is mounted.

The server has an optional parameter, `./server --prefix /some/other/path`, if supplied all recieved invocations of `lpr` will have the files prefixed with this path. This is intended to be the path on the host where the volume is mounted

Beware that the server is completly unsecure, you can print any file you want, and there are no permissions. It should only be available to trusted sources. Even clients are not exactly secure, as without a `prefix` the paths are not checked to exist, and could contain `..` to set the server to `/` even with appending a prefix

## Configuration

You can specify `--listen <ip>` and `--port <port>` for the server to tell on what interface to listen and what port. The default port can be changed in the source file if you don't want to specify it for both server & client

You can also specify `--port <port>` for the client

## Building

To build the client `cargo build --bin client --features reqwest`, to build the server `cargo build --bin server --features="warp tokio/process"`.
