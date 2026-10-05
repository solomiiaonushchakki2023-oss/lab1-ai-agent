# Rust project skill

Use this skill when creating or maintaining a simple Cargo-based Rust CLI project.

Guidelines:
- Keep the project as a standard Cargo binary crate with a `Cargo.toml` manifest and a `src/main.rs` entry point.
- Favor a simple structure: one crate, small modules only when needed, and minimal dependencies.
- Prefer idiomatic Rust patterns and clear naming.
- For CLI tools, parse command-line arguments directly using `std::env::args()` when the project is intentionally small.
- Keep the implementation focused and easy to read; avoid unnecessary abstraction.
- Validate with `cargo test` for correctness and `cargo run -- <args>` for basic behavior when needed.

This skill is intended for small laboratory or teaching projects that should remain easy to understand and maintain.
