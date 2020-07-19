# LPR Proxy

This program is really insecure, I think it can overwrite arbitrary files if the server is run as root. It can excute arbitrary commands anyway. The only use is if you can control what is connecting to the HTTP server, and be sure of the inputs

There are two binaries, `client` and `server` that should be self expanatory. The client has exactly the same arguments as `lpr`, it is meant as a replacement
