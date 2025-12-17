@ -1,458 +1,73 @@
# rs-odyssey

# Modern Rust Hobby Project Roadmap

A 6-project roadmap to progressively learn Rust concepts, from basic networking to a complete CLI WebRTC app. Each project focuses on specific Rust skills, ownership, borrowing, error handling, and concurrency, culminating in a real-world networking application.

## **Learning Rules**

- **No AI/LLM assistance** is allowed for writing code or generating solutions
- **Very basic tab completion** or code snippets in the IDE are allowed
- **AI can be used** for code review, explanations, or as a "Google" to look up syntax and concepts
- **The goal** is to learn by thinking and solving problems directly

---

## **Project 1 — TCP/UDP Echo Server**
**Duration:** 1–2 weeks

### Goal
Build a server that can accept messages from clients and reply correctly, handling multiple clients sequentially. Learn basic networking and Rust's ownership system.

### Success Criteria
- Server receives a message from a client and replies with the same message prefixed with a timestamp
- UDP version counts packets received from each client and reports statistics
- Handles client disconnects gracefully without panicking
- Proper error handling using `Result` and `Option` types
- Resources (sockets, buffers) are cleaned up automatically via RAII (Drop trait)

### Concepts Practiced
- `std::net` for TCP/UDP sockets
- `Result<T, E>` and `Option<T>` for error handling
- Ownership and borrowing with buffers (`Vec<u8>`, `String`)
- Pattern matching for error cases
- The `Drop` trait for automatic cleanup
- String handling and conversions

### Potential Features / Extensions
- Echo server that prefixes responses with timestamps using `chrono`
- Logging connection attempts using `log` and `env_logger`
- Timeout handling for idle clients using `std::time`
- Statistics tracking (bytes transferred, connection duration)
- Configurable port and buffer sizes via command-line args (`clap`)
- Simple protocol with message framing (length prefix)
- Custom error types using `thiserror`

### Technical Challenges
- Understanding when to use `&str` vs `String` vs `&[u8]` vs `Vec<u8>`
- Handling the `Result` type without unwrapping everywhere
- Managing buffer ownership across read/write operations
- Preventing panics on client disconnects
- Handling partial reads and writes correctly
- Understanding lifetimes in function signatures

### Learning Resources to Explore
- The Rust Book chapters on ownership, borrowing, and error handling
- `std::net` documentation
- Error handling patterns (`?` operator, custom error types)
- Buffer management strategies

---

## **Project 2 — Async Chat Server**
**Duration:** 2–3 weeks

### Goal
Build a concurrent chat server using async/await. Learn Rust's async model and shared state management.

### Success Criteria
- Each client runs concurrently using async tasks
- Server broadcasts messages to all connected clients
- Clients can send private messages using unique identifiers
- Server maintains safe access to shared resources using `Arc` and `Mutex` or `RwLock`
- Server shuts down cleanly without panicking or hanging

### Concepts Practiced
- `async`/`await` syntax and futures
- `tokio` or `async-std` runtime
- `Arc<Mutex<T>>` for shared mutable state
- `mpsc` channels for message passing
- Spawning and managing async tasks
- Interior mutability patterns

### Potential Features / Extensions
- Unique usernames or IDs for clients
- Basic commands: `/nick`, `/quit`, `/list`, `/whisper`
- Message history (last N messages) stored in shared state
- Logging messages with timestamps to file asynchronously
- Client reconnection handling
- Room/channel support for multiple chat groups
- Presence notifications (user joined/left)
- Rate limiting per client
- WebSocket support using `tokio-tungstenite`

### Technical Challenges
- Understanding when to use `Arc<Mutex<T>>` vs `mpsc` channels
- Avoiding deadlocks with async locks
- Managing client connections and cleanup on disconnect
- Broadcasting messages efficiently without blocking
- Graceful shutdown of all tasks
- Handling backpressure with slow clients
- Understanding `Send` and `Sync` trait bounds

### Learning Resources to Explore
- Tokio tutorial and documentation
- Async Rust book
- Channel patterns (mpsc, broadcast, watch)
- Shared state patterns in async code
- `Pin` and `Unpin` for understanding futures

---

## **Project 3 — File Processing Pipeline**
**Duration:** 2 weeks

### Goal
Build a CLI tool that reads, processes, and writes files safely and efficiently. Learn iterators, traits, and zero-cost abstractions.

### Success Criteria
- Reads and writes files using standard library types
- Processes large files in chunks without loading everything into memory
- Supports transformations (uppercase, compression, encryption) via a trait
- Proper error propagation using `Result` and custom error types
- Memory-efficient streaming for files larger than available RAM
- Uses iterators for functional-style processing

### Concepts Practiced
- File I/O with `std::fs` and `std::io`
- Iterator adapters (`map`, `filter`, `fold`, etc.)
- Defining and implementing custom traits
- Error propagation with `?` operator
- Custom error types with `thiserror` or `anyhow`
- Buffered I/O for performance
- Zero-copy operations where possible

### Potential Features / Extensions
- Streaming files in chunks with configurable buffer sizes
- Transform file content (uppercase, compress with `flate2`, simple XOR encryption)
- Compare files line-by-line and show diffs
- Progress reporting for large file operations using `indicatif`
- Parallel processing of independent chunks using `rayon`
- Support multiple input/output formats
- Pipeline composition (chain multiple transformations)
- Memory-mapped file I/O using `memmap2`

### Technical Challenges
- Designing a trait system for composable transformations
- Efficient buffer reuse and memory management
- Handling partial reads/writes and interruptions
- Error handling across iterator chains
- Understanding trait objects vs generics for plugins
- Balancing readability with performance
- Testing with various file sizes and edge cases

### Learning Resources to Explore
- Iterator trait and adapters
- Trait design patterns (Strategy, Builder)
- Error handling best practices (`thiserror` vs `anyhow`)
- `std::io::BufReader` and `BufWriter`
- Zero-copy techniques

---

## **Project 4 — Generic Data Structure Library**
**Duration:** 2–3 weeks

### Goal
Create a small library of generic data structures and algorithms. Master generics, lifetimes, and trait bounds.

### Success Criteria
- Implements generic data structures (e.g., LRU cache, trie, circular buffer)
- Supports multiple types without code duplication
- Uses trait bounds to constrain generic parameters appropriately
- Provides iterator implementations for custom structures
- Compiles with clear error messages for misuse
- Includes comprehensive unit tests

### Concepts Practiced
- Generic types with trait bounds
- Lifetime parameters and annotations
- Associated types in traits
- Implementing standard library traits (`Iterator`, `From`, `Debug`, etc.)
- `PhantomData` for marker types
- Const generics for array sizes
- Where clauses for complex bounds

### Potential Features / Extensions
- LRU cache with configurable capacity
- Trie for string storage and prefix searching
- Circular buffer with iterator support
- Generic Result type for domain-specific errors
- Type-safe builder pattern using the typestate pattern
- Serialization support with `serde`
- Generic graph structure with traversal algorithms
- Skip list implementation
- Compile-time validated types (e.g., NonZeroU32-style wrappers)

### Technical Challenges
- Understanding when lifetimes are needed vs inferred
- Designing ergonomic APIs with complex generic bounds
- Implementing double-ended iterators correctly
- Handling lifetime variance correctly
- Understanding the difference between `'static` and owned data
- Making error messages understandable
- Balancing generality with usability
- Understanding HRTBs (Higher-Rank Trait Bounds)

### Learning Resources to Explore
- The Rust Book chapter on lifetimes and generics
- Rust by Example for trait patterns
- `std::iter::Iterator` trait documentation
- The Rustonomicon for advanced topics
- Typestate pattern for compile-time state machines

---

## **Project 5 — Event-Driven Task Scheduler**
**Duration:** 3 weeks

### Goal
Build an event-driven task scheduler with async support. Learn closures, trait objects, and advanced async patterns.

### Success Criteria
- Implements an event system where actions trigger handlers
- Supports scheduling of tasks with delays or conditions
- Handlers can be closures or trait objects
- Tasks and events are handled safely across async boundaries
- Optional: integrates with async runtime for true concurrency
- Clean shutdown cancels all pending events

### Concepts Practiced
- Closures and capture semantics (`Fn`, `FnMut`, `FnOnce`)
- Trait objects (`dyn Trait`) for type erasure
- `Box<dyn Fn()>` for storing callbacks
- Async tasks and spawning with `tokio::spawn`
- `select!` macro for handling multiple futures
- Channels for event communication
- Weak references with `Weak<T>` to prevent cycles

### Potential Features / Extensions
- CLI task scheduler with delayed tasks and cron-like syntax
- Event priorities and ordering with custom priority queue
- Cancellable tasks using `tokio::sync::CancellationToken`
- Event chaining (trigger one event after another)
- Conditional events (run if predicate is true)
- Recurring tasks with intervals
- Plugin system using dynamic loading (`libloading`)
- State machine implementation for complex workflows
- Event logging and replay for debugging

### Technical Challenges
- Understanding closure trait bounds (`Fn` vs `FnMut` vs `FnOnce`)
- Making closures `Send + 'static` for async contexts
- Preventing memory leaks with cyclic references
- Handling panics inside event handlers
- Coordinating shutdown across multiple tasks
- Type erasure with `dyn Trait` and object safety rules
- Understanding why some traits aren't object-safe
- Testing async code with time manipulation

### Learning Resources to Explore
- Closure types and when to use each
- Object safety rules for trait objects
- Async cancellation patterns
- `tokio::time` for timers and intervals
- Pin and Unpin for self-referential types

---

## **Project 6 — Complete CLI WebRTC App**
**Duration:** 4–6 weeks

### Goal
Build a fully working CLI WebRTC application supporting peer-to-peer messaging and file transfer. Apply all prior knowledge to a real-world networking project.

### Success Criteria
- Peer-to-peer signaling and connection establishment work reliably
- Data channels can send and receive text messages and files
- Optional: audio streaming between peers
- Safe handling of callbacks and asynchronous events using Rust's type system
- All resources are managed correctly (no leaks, proper cleanup)
- Large messages and files are transferred efficiently using streaming
- Handles network interruptions gracefully and recovers connections
- Clean shutdown with no panics or hanging tasks

### Concepts Practiced (Rust-Specific)
- Integration with WebRTC library (`webrtc-rs` or FFI bindings)
- Peer-to-peer signaling and session management (SDP, ICE)
- Data channels for messages and files
- Async Rust for concurrent connections
- `Arc` and `Mutex`/`RwLock` for shared peer state
- Streaming large files with async iterators or channels
- Error handling across FFI boundaries if using C libraries
- Lifetime management with callbacks
- Type-safe state machines for connection states

### Potential Features / Extensions
- CLI chat between peers with multiple simultaneous connections
- File transfer with chunked buffers and progress reporting using `indicatif`
- Optional audio streaming via `cpal` or similar library
- Comprehensive logging with `tracing` and configurable levels
- Configuration file support using `serde` and `toml`/`yaml`
- NAT traversal handling (STUN, TURN)
- Connection state management using typestate pattern
- Automatic reconnection on network interruptions
- Peer discovery on local network using mDNS
- TUI interface with `ratatui` for better visualization
- Performance metrics (bandwidth, latency, packet loss)
- E2E encryption using `ring` or `rustls`

### Technical Challenges (Rust-Specific)
- Managing peer lifetimes safely with `Arc` and `Weak`
- Handling callbacks from C FFI in safe Rust
- Coordinating multiple async operations without races
- Performance tuning for large files (zero-copy where possible)
- Wrapping unsafe FFI code safely
- Testing peer-to-peer connections (network simulation)
- Debugging timing-sensitive async issues
- Ensuring `Send + Sync` bounds are satisfied
- Graceful error recovery without unwinding across FFI

### Learning Resources to Explore
- WebRTC fundamentals (signaling, ICE, DTLS, SRTP)
- `webrtc-rs` crate documentation
- NAT traversal techniques
- Audio codec integration in Rust
- Async patterns for P2P networking
- FFI patterns if using C WebRTC libraries
- `unsafe` code guidelines and best practices

### Suggested Implementation Phases
1. **Phase 1:** Basic signaling and connection establishment
2. **Phase 2:** Data channel text messaging
3. **Phase 3:** File transfer with chunking and progress
4. **Phase 4:** Multi-peer support and connection management
5. **Phase 5:** Audio streaming (optional)
6. **Phase 6:** Polish, error handling, and performance optimization

---

## **Cross-cutting Rust Skills**

| Skill | Projects Practiced | Mastery Level |
|-------|-----------------|---------------|
| Ownership & borrowing | All projects | Beginner → Expert |
| Error handling (`Result`, `Option`) | All projects | Beginner → Advanced |
| Pattern matching | All projects | Consistent practice |
| Lifetimes | 3, 4, 6 | Intermediate → Advanced |
| Traits & generics | 3, 4, 5, 6 | Beginner → Advanced |
| Async/await | 2, 5, 6 | Beginner → Advanced |
| Closures & trait objects | 5, 6 | Intermediate → Advanced |
| Networking fundamentals | 1, 2, 6 | Beginner → Advanced |
| Iterator patterns | 3, 4 | Intermediate → Advanced |
| Unsafe & FFI (optional) | 6 | Advanced |

---

## **General Development Practices**

Throughout all projects, practice these essential habits:

### Code Quality
- Write idiomatic Rust following community conventions
- Use `clippy` for linting and suggestions
- Use `rustfmt` for consistent formatting
- Keep functions small and focused
- Leverage the type system for correctness

### Testing
- Write unit tests with `#[test]` and `cargo test`
- Use `#[cfg(test)]` modules for test-only code
- Test edge cases, error paths, and panics with `#[should_panic]`
- Consider property-based testing with `proptest` or `quickcheck`
- Use `cargo-tarpaulin` or `cargo-llvm-cov` for coverage

### Debugging
- Learn `rust-lldb` or `rust-gdb` basics
- Use `dbg!()` macro for quick debugging
- Enable backtraces with `RUST_BACKTRACE=1`
- Use `tracing` or `log` for structured logging
- Keep a development journal of tricky bugs

### Build System
- Organize code with `cargo` workspaces for multi-crate projects
- Use features flags for optional functionality
- Configure different profiles (dev, release, bench)
- Use `cargo-watch` for automatic rebuilding
- Consider `cargo-nextest` for faster test execution

### Version Control
- Commit frequently with clear messages
- Use `.gitignore` to exclude `target/` and `Cargo.lock` (for libraries)
- Create branches for experimental features
- Tag completed milestones
- Review your own diffs before committing

---

## **Recommended Rust Editions**

Start with **Rust 2021 edition** as it's the current stable version:

### Essential Language Features
- Ownership, borrowing, and lifetimes (core to everything)
- Pattern matching and destructuring
- `Result` and `Option` for error handling
- Traits and trait bounds
- Closures and iterators
- `async`/`await` for concurrency

### Important Standard Library Types
- `Vec<T>`, `String`, `HashMap<K, V>`, `HashSet<T>`
- `Arc<T>`, `Rc<T>` for shared ownership
- `Mutex<T>`, `RwLock<T>` for interior mutability
- `Box<T>` for heap allocation
- `RefCell<T>`, `Cell<T>` for single-threaded interior mutability

### Key Crates to Learn
- **async:** `tokio`, `async-std`
- **error handling:** `thiserror`, `anyhow`
- **CLI:** `clap`, `indicatif`
- **serialization:** `serde`, `serde_json`, `toml`
- **logging:** `log`, `env_logger`, `tracing`
- **testing:** `proptest`, `criterion` (benchmarking)

---

## **Learning Path Summary**

This roadmap provides **progressive mastery** through increasingly complex projects:

1. **Projects 1-2:** Foundation (ownership, borrowing, error handling, async)
2. **Projects 3-4:** Intermediate (traits, generics, lifetimes, iterators)
3. **Projects 5-6:** Advanced (trait objects, complex async, real-world application)

Each project builds on previous skills while introducing new concepts. By the end, you'll have:
- A portfolio of working Rust applications
- Deep understanding of ownership, borrowing, and lifetimes
- Confidence to tackle real-world Rust projects
- Strong foundation for further specialization (systems programming, embedded, web services)
- Experience with the Rust ecosystem and tooling

**Estimated Total Time:** 16-23 weeks (4-6 months)

The final WebRTC project serves as a capstone, demonstrating mastery of all prior concepts in a challenging, practical application. 

## **Key Differences from C++**

As you transition from C++ concepts to Rust:

- **RAII → Drop trait:** Automatic cleanup is similar but enforced by the compiler
- **Smart pointers → Ownership:** `Box`, `Rc`, `Arc` serve similar purposes but with compile-time checks
- **Move semantics → Default moves:** Rust moves by default; you opt into copying
- **Const correctness → Borrowing:** `&` vs `&mut` is enforced at compile time
- **Templates → Generics + Traits:** More explicit constraints, better error messages
- **Threads → Async tasks:** Async/await is the preferred concurrency model for I/O
- **Exceptions → Result:** No exceptions; explicit error handling with `Result<T, E>`

Good luck, and remember: **fighting the borrow checker is where the learning happens!** The compiler is your teacher—read its error messages carefully.