# Repository Guidelines

## Project Structure & Module Organization
- Core sieve logic lives in `src/prime_sieve.rs`; CLI entrypoint is `src/main.rs`; GUI wrapper is `src/gui_main.rs`.
- Library exports are re-exported via `src/lib.rs` for tests and potential embedding.
- Integration tests are under `tests/prime_sieve_tests.rs`; test artifacts stay in `target/`.
- Notes and docs: `README.md`, `agent.md`, and localized docs in `Chinese_note/` and `English_note/`.

## Build, Test, and Development Commands
- `cargo build` / `cargo build --release`: compile binaries `prime-cli` and `prime-gui`.
- `cargo run --bin prime-cli --release`: prompt for an upper bound and run the segmented sieve.
- `cargo run --bin prime-gui --release`: launch the egui-based GUI (ensure a windowing environment is available).
- `cargo test`: run integration tests in `tests/prime_sieve_tests.rs`.
- `cargo fmt` then `cargo clippy -- -D warnings`: format and lint before pushing.

## Coding Style & Naming Conventions
- Rust 2021 edition; default 4-space indentation and `snake_case` for functions/variables, `CamelCase` for types.
- Prefer pure functions over shared mutable state; keep parallelism explicit via Rayon iterators.
- Avoid panics in new code paths; return `Result` where user input or IO is involved.
- Keep modules small: sieve internals in `prime_sieve.rs`; UI/IO wiring in binaries.

## Testing Guidelines
- Unit tests should live beside logic when small; larger/IO-heavy cases belong in `tests/`.
- Name tests descriptively with behavior focus, e.g., `sieves_small_ranges`, `handles_large_chunks`.
- For new algorithms or optimizations, add edge cases around low bounds (0–10) and chunk-size changes.
- If a change could affect performance correctness, include a sanity assert on counts against known ranges.

## Commit & Pull Request Guidelines
- Follow the existing short, imperative-style summaries seen in `git log` (e.g., “fix sieve bounds”); keep subjects under ~72 chars.
- Include what changed and why in the PR description; link issues or tasks when applicable.
- Document testing performed (commands run and platforms). For GUI tweaks, attach a brief note on observed behavior; screenshots optional but helpful.
- Keep commits focused; prefer small, reviewable diffs over large mixed changes.

## Performance & Safety Tips
- Be cautious with very large upper bounds: memory and runtime scale quickly. Test on modest ranges before scaling up.
- When adjusting chunk sizes or parallel settings, measure both correctness and throughput; avoid unbounded allocations.
## Answer users in **Chinese**
