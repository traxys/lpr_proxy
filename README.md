# LPR Proxy

This client/server is for forwarding `lpr` calls from one machine to the other, where each machine has the same sets of files.
The main use is when you want to call `lpr` from a docker but you don't want to bother managing `CUPS` in that docker

The client has the same options as `lpr`, with two more `--remote <ip>` wich is the IP of the remote server, and optionally `--prefix /some/path`, wich will remove from all invocations the said path, having canonicalized the paths first. This is intended to be the path in the docker where a volume is mounted.

The server has an optional positional parameter, `./server /some/other/path`, if supplied all recieved invocations of `lpr` will have the files prefixed with this path. This is intended to be the path on the host where the volume is mounted

Beware that the server is completly unsecure, you can print any file you want, and there are no permissions. It should only be available to trusted sources. Even clients are not exactly secure, as without a `prefix` the paths are not checked to exist, and could contain `..` to set the server to `/` even with appending a prefix
