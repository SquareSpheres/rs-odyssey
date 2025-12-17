# Project 1: TCP/UDP Echo Server

**Status:** Not Started  
**Duration:** 1-2 weeks

## Goal
Build a simple TCP and UDP echo server that echoes back any messages received from clients.

## Success Criteria
- [ ] TCP server accepts connections and echoes messages
- [ ] UDP server receives packets and echoes them back
- [ ] Proper Result/Option error handling throughout
- [ ] Handle client disconnects gracefully
- [ ] No unwrap() in production code

## Concepts Practiced
- Ownership and borrowing
- Result and Option types
- Pattern matching
- std::net (TcpListener, UdpSocket)
- Error handling with thiserror/anyhow

## Build & Run

```bash
# Build
cargo build -p project1-echo-server

# Run
cargo run -p project1-echo-server

# Test
cargo test -p project1-echo-server

# With clippy
cargo clippy -p project1-echo-server
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*

