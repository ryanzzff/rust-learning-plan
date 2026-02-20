# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

This is a **Rust learning repository** — a hands-on workspace for learning Rust from scratch. The learning path is documented in `LEARNING_PLAN.md` with 10 progressive stages, each with exercises that build on prior concepts.

## Common Commands

```bash
# Project setup (run once per exercise)
cargo new <project-name>       # Create a new binary project
cargo new --lib <project-name> # Create a new library project

# Development cycle
cargo build                    # Compile the project
cargo run                      # Build and run
cargo test                     # Run all tests
cargo test <test_name>         # Run a single test
cargo check                    # Fast compile check (no binary output)
cargo clippy                   # Lint for idiomatic Rust
cargo fmt                      # Format code

# Learning tools
cargo doc --open               # Generate and view docs for dependencies
rustup doc                     # Open local Rust documentation
```

## Repository Structure

```
LEARNING_PLAN.md          # Master plan with 10 stages and status tracking
docs/                     # Per-stage reference notes and insights
  stage-01-basics.md
  stage-02-ownership.md
  ...
exercises/                # Exercise projects (one cargo project per stage)
  stage-01-guessing-game/
  stage-02-string-tool/
  ...
```

## Teaching Approach

- This learner is **new to Rust** — explain Rust-specific concepts (ownership, borrowing, lifetimes) with care
- Follow the stage order in `LEARNING_PLAN.md` — each stage builds on prior ones
- Use the **Learning output style**: provide `Learn by Doing` prompts for key decisions and `Insight` blocks for educational context
- Exercises should be built incrementally: scaffold first, then ask the learner to implement core logic
- Always run `cargo clippy` and `cargo test` after changes
- Update stage status in `LEARNING_PLAN.md` as exercises are completed
- After completing each exercise, add a **"Lessons Learned"** section to the stage's doc file (`docs/stage-XX-*.md`) capturing practical insights discovered during the exercise — compiler tricks, idiomatic patterns, gotchas encountered
