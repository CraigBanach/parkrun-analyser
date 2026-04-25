# ADR 0002: Ingestion Error Handling and Parsing Boundary

- Status: Accepted
- Date: 2026-04-25
- Decision Makers: Project maintainers

## Context

The analyser reads input from files that may contain malformed rows. We need behavior that is robust for real-world data and still supports strict validation for future workflows.

We also need a clear boundary between:

- low-level file and parsing concerns, and
- application-level analysis and reporting concerns.

## Decision

### Error handling mode (default)

Default ingestion mode is **tolerant**:

- Invalid rows are skipped.
- Warnings are recorded for skipped rows.
- Analysis continues using valid rows.
- A warning summary is reported at the end.

### Strict mode (planned)

A future optional strict mode will be added:

- Parsing stops at first invalid row.
- Command exits with non-zero status.
- Error location and reason are reported.

Strict mode is deferred and tracked as future work.

### Parsing boundary

`io` is responsible for converting file input into typed domain records.

- `io` reads and parses rows.
- `app` consumes typed records only.
- `app` does not parse raw strings.

This keeps business logic independent from file format mechanics.

## Consequences

### Positive

- Better user experience on noisy datasets (default path keeps running).
- Stable architecture for scaling to larger files.
- Clear ownership of parsing behavior and errors.
- Easier test strategy: parse tests and analysis tests can be separate.

### Trade-offs

- Tolerant mode may hide data quality issues if warnings are ignored.
- Requires consistent warning reporting to preserve trust.

## Guardrails

- Always report skipped-row count.
- Always report at least a small sample of warning details.
- Keep strict mode behavior documented once implemented.

## Implementation Notes

- Keep top-level process exit and user-facing error printing in `main`.
- Keep `app` focused on orchestration and aggregation.
- Keep parsing and file details in `io`.
