# Project 5: Event-Driven Task Scheduler

**Status:** Not Started  
**Duration:** 3 weeks

## Goal
Build an async task scheduler with callback-based event handling using closures and trait objects.

## Success Criteria
- [ ] Closure-based task callbacks
- [ ] Trait objects for type erasure
- [ ] Async task scheduling
- [ ] Clean task cancellation
- [ ] Event-driven architecture

## Concepts Practiced
- Closures and Fn traits (Fn, FnMut, FnOnce)
- Trait objects (Box<dyn Trait>)
- Type erasure patterns
- Async task management
- Dynamic dispatch

## Build & Run

```bash
# Build
cargo build -p project5-event-scheduler

# Run
cargo run -p project5-event-scheduler

# Test
cargo test -p project5-event-scheduler

# With clippy
cargo clippy -p project5-event-scheduler
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*

