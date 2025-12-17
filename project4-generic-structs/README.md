# Project 4: Generic Data Structure Library

**Status:** Not Started  
**Duration:** 2-3 weeks

## Goal
Build a collection of generic data structures (LRU cache, priority queue, etc.) with proper lifetime annotations.

## Success Criteria
- [ ] Generic types with trait bounds
- [ ] Correct lifetime annotations
- [ ] Iterator implementations
- [ ] Associated types where appropriate
- [ ] Benchmarks comparing to std collections

## Concepts Practiced
- Generic types with trait bounds
- Lifetime annotations
- Associated types
- Where clauses
- Iterator trait implementation
- Benchmark writing

## Build & Run

```bash
# Build
cargo build -p project4-generic-structs

# Test
cargo test -p project4-generic-structs

# Run benchmarks
cargo bench -p project4-generic-structs

# With clippy
cargo clippy -p project4-generic-structs
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*

