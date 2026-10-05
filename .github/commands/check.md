# check

Purpose: check formatting and code quality using appropriate Cargo tools.

Instructions:
- Run `cargo fmt --check` to verify formatting.
- Run a relevant quality check such as `cargo clippy --all-targets --all-features -- -D warnings` when available and appropriate for the project.
- If either step reports issues, fix the root cause and rerun the check.
- Keep the checks focused on code quality and maintainability without broad, unrelated cleanup.

References:
- Skill: `rust-project`
- Skill: `testing`
- Related command: `init`
