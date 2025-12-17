# Rust Learning Monorepo Scaffolding Prompt

Create a complete directory structure and configuration files for a Rust learning workspace called "rs-odyssey". Follow these exact specifications:

## Directory Structure

Create the following directory tree:

```
rs-odyssey/
├── .github/
│   └── workflows/
├── docs/
│   └── reflections/
├── shared/
│   └── src/
├── project1-echo-server/
│   ├── src/
│   ├── tests/
│   └── examples/
├── project2-async-chat/
│   ├── src/
│   └── tests/
├── project3-file-pipeline/
│   ├── src/
│   ├── tests/
│   └── sample_files/
├── project4-generic-structs/
│   ├── src/
│   ├── tests/
│   ├── examples/
│   └── benches/
├── project5-event-scheduler/
│   ├── src/
│   └── tests/
├── project6-webrtc-cli/
│   ├── src/
│   ├── tests/
│   └── config/
└── scripts/
```

## Files to Create

### Root Level Files

**1. `.gitignore`**
```gitignore
# Rust
/target
**/*.rs.bk
*.pdb
Cargo.lock  # Remove this line for applications

# IDE
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store

# Testing
*.profraw
*.profdata
```

**2. `rustfmt.toml`**
```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Max"
```

**3. `Cargo.toml`** (Workspace)
```toml
[workspace]
members = [
    "shared",
    "project1-echo-server",
    "project2-async-chat",
    "project3-file-pipeline",
    "project4-generic-structs",
    "project5-event-scheduler",
    "project6-webrtc-cli",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"

[workspace.dependencies]
# Common dependencies
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
lto = true
```

**4. `README.md`** (Root)
```markdown
# rs-odyssey

> A progressive Rust learning journey through 6 hands-on projects

## 🎯 Overview

This repository documents my journey learning Rust through increasingly complex projects, from basic networking to a full WebRTC application.

**Learning Rule:** No AI-generated code – only self-directed learning, research, and problem-solving.

## 📚 Projects

| # | Project | Status | Duration | Key Concepts |
|---|---------|--------|----------|--------------|
| 1 | [Echo Server](project1-echo-server) | ⚪ Not Started | 1-2 weeks | Ownership, Error Handling |
| 2 | [Async Chat](project2-async-chat) | ⚪ Not Started | 2-3 weeks | Async/Await, Arc/Mutex |
| 3 | [File Pipeline](project3-file-pipeline) | ⚪ Not Started | 2 weeks | Iterators, Traits |
| 4 | [Generic Structs](project4-generic-structs) | ⚪ Not Started | 2-3 weeks | Generics, Lifetimes |
| 5 | [Event Scheduler](project5-event-scheduler) | ⚪ Not Started | 3 weeks | Closures, Trait Objects |
| 6 | [WebRTC CLI](project6-webrtc-cli) | ⚪ Not Started | 4-6 weeks | Integration, Real-world App |

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/rs-odyssey.git
cd rs-odyssey

# Build all projects
cargo build --workspace

# Run tests
cargo test --workspace

# Run a specific project
cargo run -p project1-echo-server

# With clippy
cargo clippy --workspace -- -D warnings
```

## 🛠️ Requirements

- Rust 2021 edition or later
- Cargo (comes with Rust)

Install Rust: https://rustup.rs/

## 📖 Documentation

- [Full Roadmap](docs/ROADMAP.md) - Complete learning path and project details
- [Learning Resources](docs/resources.md) - Books, articles, and references
- [Progress Journal](PROGRESS.md) - My learning progress and reflections

## 📝 Learning Rules

- ❌ No AI-generated code or solutions
- ✅ Basic IDE autocomplete allowed
- ✅ AI for explanations, code review, and research
- ✅ Goal: Learn by thinking and solving problems directly

## 🤝 Feedback

This is a personal learning repository. Feedback and suggestions are welcome via issues!

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

---

**Started:** [Date]  
**Current Project:** Project 1 - TCP/UDP Echo Server
```

**5. `PROGRESS.md`**
```markdown
# Learning Progress

Track your journey through each project, challenges faced, and lessons learned.

## Project 1: TCP/UDP Echo Server
**Status:** ⚪ Not Started  
**Started:** -  
**Completed:** -

### Goals
- [ ] Build TCP echo server
- [ ] Build UDP packet counter
- [ ] Proper Result/Option error handling
- [ ] Handle client disconnects gracefully

### Learning Log
*Document your progress here*

---

## Project 2: Async Chat Server
**Status:** ⚪ Not Started

### Goals
- [ ] Async/await implementation
- [ ] Broadcast messaging with Arc/Mutex
- [ ] Private messaging
- [ ] Graceful shutdown

### Learning Log
*Coming soon...*

---

## Project 3: File Processing Pipeline
**Status:** ⚪ Not Started

### Goals
- [ ] Iterator-based processing
- [ ] Custom trait implementation
- [ ] Chunked file reading
- [ ] Error propagation

### Learning Log
*Coming soon...*

---

## Project 4: Generic Data Structure Library
**Status:** ⚪ Not Started

### Goals
- [ ] Generic types with trait bounds
- [ ] Lifetime annotations
- [ ] Iterator implementation
- [ ] Associated types

### Learning Log
*Coming soon...*

---

## Project 5: Event-Driven Task Scheduler
**Status:** ⚪ Not Started

### Goals
- [ ] Closure-based callbacks
- [ ] Trait objects for type erasure
- [ ] Async task scheduling
- [ ] Clean cancellation

### Learning Log
*Coming soon...*

---

## Project 6: Complete CLI WebRTC App
**Status:** ⚪ Not Started

### Goals
- [ ] Peer-to-peer signaling
- [ ] Data channel messaging
- [ ] File transfer
- [ ] Connection management

### Learning Log
*Coming soon...*

---

## Overall Statistics

- **Total Time Invested:** 0 hours
- **Projects Completed:** 0/6
- **Tests Written:** 0
- **Clippy Warnings:** 0
- **Concepts Mastered:** []
```

**6. `LICENSE`**
```
MIT License

Copyright (c) 2024 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Documentation Files

**7. `docs/ROADMAP.md`**
```markdown
# Rust Learning Roadmap

[Copy the full Rust roadmap content from the previous artifact here]
```

**8. `docs/resources.md`**
```markdown
# Learning Resources

## Books
- **"The Rust Programming Language"** (The Book) - Official guide
- **"Rust for Rustaceans"** by Jon Gjengset - Intermediate/advanced
- **"Programming Rust"** by Jim Blandy & Jason Orendorff - Comprehensive
- **"Asynchronous Programming in Rust"** - Async book

## Online Resources

### Official
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [std docs](https://doc.rust-lang.org/std/)

### Community
- [r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)
- [This Week in Rust](https://this-week-in-rust.org/)

## Project-Specific Resources

### Project 1: Networking
- std::net documentation
- Error handling chapter in The Book
- Ownership chapter in The Book

### Project 2: Async
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- Async book
- Jon Gjengset's async streams video

### Project 3: Iterators & Traits
- Iterator chapter in The Book
- Trait chapter in The Book
- Iterator cheat sheet

### Project 4: Generics & Lifetimes
- Lifetimes chapter in The Book
- Jon Gjengset's lifetime video
- "Rust for Rustaceans" chapters 1-3

### Project 5: Closures
- Closures chapter in The Book
- Trait objects documentation
- Pin and Unpin explanation

### Project 6: WebRTC
- webrtc-rs documentation
- WebRTC fundamentals
- Async patterns in production

## Tools

- **cargo-watch** - Auto-rebuild on changes
- **cargo-expand** - See macro expansions
- **cargo-flamegraph** - Profiling
- **cargo-audit** - Security vulnerabilities
- **cargo-nextest** - Faster test runner
```

**9. `docs/reflections/README.md`**
```markdown
# Project Reflections

This directory contains detailed reflections written after completing each project.

## Template

```markdown
# Project N: [Project Name]

**Completed:** [Date]  
**Duration:** [Weeks]  
**Total Hours:** ~[Estimate]

## Overview
What you built and key features.

## What I Learned
- Ownership patterns: ...
- Error handling: ...
- Async patterns: ...

## Challenges & Solutions
### Challenge 1: Fighting the Borrow Checker
**Problem:** 
**Solution:** 
**Lesson:** 

## Design Decisions
- Why I chose X over Y
- Trade-offs made

## Code Quality
- Tests written: N
- Clippy warnings: 0
- Documentation: [Coverage]

## Favorite Code Snippet
```rust
// Your proudest piece of Rust
```

## Resources Used
- Documentation pages
- Blog posts
- Videos

## Next Steps
What to focus on next.
```
```

### GitHub Actions

**10. `.github/workflows/ci.yml`**
```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

    runs-on: ${{ matrix.os }}

    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache target
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-target-${{ matrix.rust }}-${{ hashFiles('**/Cargo.lock') }}

    - name: Build
      run: cargo build --workspace --verbose

    - name: Run tests
      run: cargo test --workspace --verbose

    - name: Run clippy
      run: cargo clippy --workspace -- -D warnings

    - name: Check formatting
      run: cargo fmt --all -- --check
```

### Scripts

**11. `scripts/setup.sh`**
```bash
#!/bin/bash
# Development environment setup

set -e

echo "=== Rust Forge Setup ==="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Install from https://rustup.rs/"
    exit 1
fi

echo "Rust version: $(rustc --version)"

# Install useful tools
echo "Installing development tools..."
cargo install cargo-watch cargo-expand cargo-audit

echo "=== Setup Complete ==="
echo "Run 'cargo build --workspace' to build all projects"
```

**12. `scripts/test-all.sh`**
```bash
#!/bin/bash
# Run all tests with coverage

set -e

echo "=== Running all tests ==="
cargo test --workspace --verbose

echo "=== Running clippy ==="
cargo clippy --workspace -- -D warnings

echo "=== Checking formatting ==="
cargo fmt --all -- --check

echo "=== All checks passed ==="
```

**13. `scripts/format.sh`**
```bash
#!/bin/bash
# Format all Rust code

cargo fmt --all
echo "Formatting complete"
```

### Per-Project Files

**14. `shared/Cargo.toml`**
```toml
[package]
name = "shared"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
# Common utilities will go here
```

**15. `shared/src/lib.rs`**
```rust
// Shared utilities across all projects
// Will be populated as needed
```

**16. `project1-echo-server/Cargo.toml`**
```toml
[package]
name = "project1-echo-server"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }
anyhow.workspace = true
thiserror.workspace = true
clap.workspace = true

[dev-dependencies]
```

**17. `project2-async-chat/Cargo.toml`**
```toml
[package]
name = "project2-async-chat"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }
tokio.workspace = true
anyhow.workspace = true
thiserror.workspace = true
clap.workspace = true

[dev-dependencies]
tokio-test = "0.4"
```

**18. `project3-file-pipeline/Cargo.toml`**
```toml
[package]
name = "project3-file-pipeline"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }
anyhow.workspace = true
thiserror.workspace = true
clap.workspace = true

[dev-dependencies]
tempfile = "3.8"
```

**19. `project4-generic-structs/Cargo.toml`**
```toml
[package]
name = "project4-generic-structs"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }

[dev-dependencies]

[[bench]]
name = "benchmarks"
harness = false
```

**20. `project5-event-scheduler/Cargo.toml`**
```toml
[package]
name = "project5-event-scheduler"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }
tokio.workspace = true
anyhow.workspace = true

[dev-dependencies]
tokio-test = "0.4"
```

**21. `project6-webrtc-cli/Cargo.toml`**
```toml
[package]
name = "project6-webrtc-cli"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
shared = { path = "../shared" }
tokio.workspace = true
anyhow.workspace = true
thiserror.workspace = true
clap.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[dev-dependencies]
```

**22. `project*/README.md`** (Template for each project)
```markdown
# Project [N]: [Project Name]

**Status:** Not Started  
**Duration:** [Estimated]

## Goal
[Project goal from roadmap]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Concepts Practiced
- Concept 1
- Concept 2

## Build & Run

```bash
# Build
cargo build -p project[N]-[name]

# Run
cargo run -p project[N]-[name]

# Test
cargo test -p project[N]-[name]

# With clippy
cargo clippy -p project[N]-[name]
```

## Design Notes
*Architectural decisions*

## Known Issues
*Track issues during development*
```

---

## Creation Instructions

1. Create all directories in the structure shown
2. Create all files with the content provided
3. Make shell scripts executable: `chmod +x scripts/*.sh`
4. Initialize git: `git init`
5. Test the setup: `cargo build --workspace`

Do NOT create any `.rs` files in src/ directories beyond `lib.rs` for shared - those will be created during development.

The result should be a complete, buildable workspace ready for implementation.
