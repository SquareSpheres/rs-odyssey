# Rust Learning Roadmap

A structured 6-project learning path from Rust fundamentals to production-ready applications.

## Project Progression

### Project 1: TCP/UDP Echo Server (1-2 weeks)
**Focus:** Ownership, borrowing, error handling fundamentals

Build a simple network server that echoes back messages. This establishes core Rust patterns.

**Key concepts:**
- Ownership and borrowing
- `Result` and `Option` handling
- Pattern matching
- Basic `std::net` usage

### Project 2: Async Chat Server (2-3 weeks)
**Focus:** Async/await, concurrency primitives

Build a multi-client chat server with broadcast and private messaging.

**Key concepts:**
- Tokio runtime and async/await
- `Arc<Mutex<T>>` for shared state
- Channel-based communication
- Graceful shutdown patterns

### Project 3: File Processing Pipeline (2 weeks)
**Focus:** Iterators, traits, and composable abstractions

Build a streaming file processor with pluggable transformations.

**Key concepts:**
- Iterator combinators
- Custom trait definitions
- Trait implementations
- Error propagation with `?`

### Project 4: Generic Data Structure Library (2-3 weeks)
**Focus:** Generics, lifetimes, advanced type system

Build a collection of generic data structures (LRU cache, priority queue, etc.).

**Key concepts:**
- Generic types with trait bounds
- Lifetime annotations
- Associated types
- Where clauses

### Project 5: Event-Driven Task Scheduler (3 weeks)
**Focus:** Closures, trait objects, dynamic dispatch

Build an async task scheduler with callback-based event handling.

**Key concepts:**
- Closures and `Fn` traits
- Trait objects (`Box<dyn Trait>`)
- Type erasure patterns
- Async task management

### Project 6: WebRTC CLI Application (4-6 weeks)
**Focus:** Integration, real-world complexity

Build a peer-to-peer CLI application using WebRTC for signaling and data channels.

**Key concepts:**
- External crate integration (webrtc-rs)
- Complex async state machines
- Error handling at scale
- Production-ready patterns

## Recommended Timeline

| Week | Focus |
|------|-------|
| 1-2 | Project 1: Echo Server |
| 3-5 | Project 2: Async Chat |
| 6-7 | Project 3: File Pipeline |
| 8-10 | Project 4: Generic Structs |
| 11-13 | Project 5: Event Scheduler |
| 14-19 | Project 6: WebRTC CLI |

**Total estimated time:** 4-5 months at moderate pace

## Success Metrics

- All tests passing (`cargo test --workspace`)
- Zero clippy warnings (`cargo clippy --workspace -- -D warnings`)
- Documented public APIs
- Clean error handling (no `unwrap()` in production code)
- Reflection written for each completed project

