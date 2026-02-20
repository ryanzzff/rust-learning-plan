# rust-learning-plan

A hands-on Rust learning path from zero to building real-world applications, structured in 10 progressive stages.

## Overview

| Stage | Topic | Exercise |
|-------|-------|----------|
| 1 | Setup & Basics | Guessing game (CLI) |
| 2 | Ownership & Borrowing | String manipulation tool |
| 3 | Structs, Enums & Matching | Task tracker (Todo CLI) |
| 4 | Error Handling | CSV file parser |
| 5 | Collections & Iterators | Word frequency counter |
| 6 | Traits & Generics | Plugin system |
| 7 | Modules & Crates | Refactor into multi-module |
| 8 | Testing | Test all previous projects |
| 9 | Concurrency | Parallel file search |
| 10 | Capstone | CLI tool / HTTP server / CRUD app |

## Repository Structure

- **[LEARNING_PLAN.md](LEARNING_PLAN.md)** — Master plan with status tracking and dependency diagrams
- **[docs/](docs/)** — Per-stage reference notes with code examples, mental models, and common pitfalls
- **exercises/** — Cargo projects for each stage (created as you progress)

## Getting Started

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version
```

Start with [Stage 1](docs/stage-01-basics.md) and work through each stage in order.
