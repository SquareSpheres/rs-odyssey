# Project 3: File Processing Pipeline

**Status:** Not Started  
**Duration:** 2 weeks

## Goal
Build a streaming file processor with pluggable transformations using iterator patterns.

## Success Criteria
- [ ] Iterator-based file processing
- [ ] Custom trait for transformations
- [ ] Chunked file reading for large files
- [ ] Proper error propagation
- [ ] Composable pipeline stages

## Concepts Practiced
- Iterator combinators (map, filter, fold)
- Custom trait definitions and implementations
- Error propagation with ?
- Generic functions
- File I/O patterns

## Build & Run

```bash
# Build
cargo build -p project3-file-pipeline

# Run
cargo run -p project3-file-pipeline

# Test
cargo test -p project3-file-pipeline

# With clippy
cargo clippy -p project3-file-pipeline
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*

