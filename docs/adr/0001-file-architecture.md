# ADR 0001: File Architecture and Module Boundaries

- Status: Accepted
- Date: 2026-04-25
- Decision Makers: Project maintainers

## Context

The project started as a single-file Rust program (`parkrun-analyser.rs`). This was useful for fast iteration, but it now mixes:

- startup and process concerns,
- argument/config parsing,
- application flow,
- and file/data I/O.

As the codebase grows, this structure will reduce readability and make testing harder. We need a file layout that keeps responsibilities clear and supports incremental refactoring.

## Decision

We will move to a `src/`-based layout with explicit boundaries:

- `src/main.rs`
  - Entry point only.
  - Handles process-level concerns (startup, top-level error reporting, exit behavior).
  - Delegates to the application layer.

- `src/config.rs`
  - Defines runtime configuration.
  - Parses and validates CLI inputs.
  - Returns structured configuration or validation errors.

- `src/app.rs`
  - Orchestrates the main application flow.
  - Uses config and calls lower-level modules.
  - Contains no process-exit behavior.

- `src/io.rs` (introduced when needed)
  - Isolates filesystem and external I/O details.
  - Keeps I/O behavior separate from app orchestration.

## Module Dependency Direction

Dependencies should flow inward:

- `main` -> `config` and `app`
- `app` -> `io` (and future domain modules)
- `config` does not depend on `app`
- `io` does not depend on `main`

This keeps the entrypoint thin and prevents process/UI concerns from leaking into core behavior.

## Error Handling Rule

- Inner modules return errors to callers.
- Only `main` decides how errors are displayed and whether to exit.

This makes non-entrypoint code easier to test and reuse.

## Naming and Growth Rules

- Prefer one responsibility per file.
- Split files when one of these appears:
  - multiple unrelated concerns,
  - hard-to-scan long functions,
  - repeated logic that suggests a dedicated module.
- Add modules by behavior (for example `analysis`, `parser`, `report`) rather than by vague utility buckets.

## Consequences

### Positive

- Faster navigation and clearer ownership of logic.
- Easier unit testing for config parsing and app behavior.
- Lower risk when changing one part of the system.

### Trade-offs

- Slightly more upfront organization.
- More files to open, requiring a stronger editor/terminal workflow.

## Migration Plan

1. Create `src/main.rs`, `src/config.rs`, and `src/app.rs`.
2. Move entrypoint logic to `main` only.
3. Move config parsing and config type to `config`.
4. Move orchestration logic to `app`.
5. Run `cargo check` after each move.
6. Add `src/io.rs` only when I/O logic begins to grow.

## Notes

This ADR documents architecture intent. It does not require a big-bang rewrite; apply it incrementally during normal feature work.
