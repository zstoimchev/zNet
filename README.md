# zNet - Peer to Peer Networking Library

zNet is a reusable peer-to-peer networking library written in Rust. Its goal is to provide a generic networking layer that can be used by different applications without coupling the library to application-specific protocols or message types.

For example, my onion routing project `zmix` needs a networking layer for managing nodes and routing messages. Similarly, `zetra` also needs the same networking part for connection with different peers on the blockchain. The question here that I got is: why not make one unified library that will do all the networking automatically?

This way, no project will need to deal with connections, or network-specific protocols, like peer discovery, etc.

The goal is to have one shared cross-platform library supported for all of the major programming languages (how is YET to be decided)...

Rust was chosen because it is proven to be fast, it is indeed lower-level programming language, like/similar to C, and of course the thrill to learn it :). Not gonna be easy but will be worth it.
soft
Still in progress, so feel free to open issues for suggestions and improvements xD
