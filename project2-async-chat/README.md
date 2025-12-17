# Project 2: Async Chat Server

**Status:** Not Started  
**Duration:** 2-3 weeks

## Goal
Build a multi-client chat server with broadcast and private messaging capabilities using async/await.

## Success Criteria
- [ ] Async server handles multiple concurrent clients
- [ ] Broadcast messages to all connected clients
- [ ] Private messaging between users
- [ ] Graceful shutdown on Ctrl+C
- [ ] Proper shared state with Arc<Mutex<T>>

## Concepts Practiced
- Tokio runtime and async/await
- Arc<Mutex<T>> for shared state
- Channel-based communication
- Graceful shutdown patterns
- Task spawning and management

## Build & Run

```bash
# Build
cargo build -p project2-async-chat

# Run server
cargo run -p project2-async-chat

# Test
cargo test -p project2-async-chat

# With clippy
cargo clippy -p project2-async-chat
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*

