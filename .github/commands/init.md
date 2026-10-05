# init

Purpose: orchestrate the full setup and validation flow for the Rust laboratory project.

Use the following sequence:
1. Use the `rust-project` skill to confirm the project is a standard Cargo binary crate and that the structure is appropriate for a simple Rust CLI.
2. Use the `build` command to compile the project with Cargo.
3. Use the `test` command to run unit tests and confirm behavior.
4. Use the `check` command to run formatting and quality checks.
5. Use the `status` command to inspect the current Git branch and repository status without modifying anything.

Expected outcome:
- The project is recognized as a valid Cargo-based Rust CLI project.
- The project builds successfully.
- The test suite passes.
- Formatting and quality checks complete without errors.
- Git status is reviewed for a safe working state.

References:
- Skills: `rust-project`, `testing`, `git-workflow`
- Commands: `build`, `test`, `check`, `status`
