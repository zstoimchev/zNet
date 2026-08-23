# zNet Roadmap

## Phase 1 — Project Setup

* [x] Create Rust library project
* [x] Initialize Git repository
* [ ] Add README and LICENSE
* [ ] Verify `cargo check`
* [ ] Verify `cargo test`

## Phase 2 — Basic TCP

* [ ] Create TCP server
* [ ] Create TCP client
* [ ] Connect client to server
* [ ] Send bytes client → server
* [ ] Send bytes server → client

## Phase 3 — Message Framing

* [ ] Add length-prefixed messages
* [ ] Implement frame sending
* [ ] Implement frame receiving
* [ ] Add maximum message size
* [ ] Test multiple messages

## Phase 4 — Peer Model

* [ ] Create `PeerId`
* [ ] Create `PeerInfo`
* [ ] Create `Connection`
* [ ] Hide raw TCP streams from public API

## Phase 5 — ZNet Core

* [ ] Create `ZNet`
* [ ] Add `start()`
* [ ] Add `stop()`
* [ ] Add `connect()`
* [ ] Track connected peers

## Phase 6 — Messaging

* [ ] Add `send(peer_id, bytes)`
* [ ] Add `broadcast(bytes)`
* [ ] Create message handler trait
* [ ] Pass received bytes to handler

## Phase 7 — Multiple Peers

* [ ] Support multiple simultaneous connections
* [ ] List connected peers
* [ ] Handle peer disconnects
* [ ] Test targeted messages
* [ ] Test broadcast

## Phase 8 — Async Networking

* [ ] Add Tokio
* [ ] Convert TCP handling to async
* [ ] Handle each peer concurrently
* [ ] Ensure slow peers do not block others

## Phase 9 — Handshake

* [ ] Define zNet handshake
* [ ] Exchange peer ID
* [ ] Exchange listening address/port
* [ ] Add protocol version
* [ ] Recognize reconnecting peers

## Phase 10 — Peer Discovery

* [ ] Track known peers
* [ ] Separate known and connected peers
* [ ] Request peers from connected nodes
* [ ] Share known peers
* [ ] Discover nodes through bootstrap peer

## Phase 11 — Connection Management

* [ ] Add bootstrap peers
* [ ] Add connection timeout
* [ ] Add retry logic
* [ ] Add reconnect logic
* [ ] Add minimum/maximum connections

## Phase 12 — Hardening

* [ ] Handle malformed frames
* [ ] Handle oversized messages
* [ ] Handle unexpected disconnects
* [ ] Implement clean shutdown
* [ ] Remove unnecessary `unwrap()`

## Phase 13 — Protocol

* [ ] Define zNet wire protocol v1
* [ ] Document frame format
* [ ] Document handshake
* [ ] Document peer discovery
* [ ] Keep application payload as raw bytes

## Phase 14 — Testing

* [ ] Add framing tests
* [ ] Add connection tests
* [ ] Add multi-peer tests
* [ ] Add discovery tests
* [ ] Add reconnect tests
* [ ] Add shutdown tests

## Phase 15 — Demo

* [ ] Create simple example application
* [ ] Run 3+ nodes locally
* [ ] Discover peers automatically
* [ ] Send message to one peer
* [ ] Broadcast to all peers

## Later

* [ ] Stabilize public API
* [ ] Investigate Java bindings
* [ ] Integrate with first Java project
* [ ] Integrate with second Java project
