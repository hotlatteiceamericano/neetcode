# CLAUDE.md

## Architecture

This is a Rust workspace (edition 2024, no external dependencies) for solving [NeetCode](https://neetcode.io) algorithm problems.

**Structure:** Each NeetCode problem category maps to a module under `src/`. Each problem is its own submodule file, declared in the category's `mod.rs`.

**Pattern:** Problem solutions are plain functions. Tests are written inline using `#[cfg(test)]` within the same file as the solution.

**Adding a problem:**
1. Create `src/<category>/<problem>.rs` with the solution function and `#[cfg(test)]` tests.
2. Add `pub mod <problem>;` to `src/<category>/mod.rs`.
3. If the category is new, also create `src/<category>/mod.rs` and add `pub mod <category>;` to `src/main.rs`.

