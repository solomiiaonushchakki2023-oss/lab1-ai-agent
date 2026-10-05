# Testing skill

Use this skill whenever code needs verification through Rust tests.

Guidelines:
- Run `cargo test` from the project root to execute the test suite.
- Read compiler and test output carefully before changing code.
- Fix the root cause of a failing test, not just the symptom.
- Add focused unit tests for logic that is easy to verify in isolation.
- Keep tests small, deterministic, and readable.
- If a test fails, inspect the exact assertion and the relevant code path before making broader edits.

This skill supports validating small Rust programs and confirming that behavior remains correct after changes.
