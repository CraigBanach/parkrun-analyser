# Data Processing Policy

This document defines operational rules for reading and analysing input data.

## 1) Default Data Quality Behavior

- Mode: tolerant
- Invalid rows: skip
- Valid rows: continue processing
- Warnings: record and report

Minimum reporting requirements:

- total rows read
- valid rows processed
- invalid rows skipped
- up to N example warnings (default target: 5)

## 2) Future Strict Mode

Strict mode is planned but not yet implemented.

Expected behavior:

- stop on first invalid row
- return non-zero exit code
- print row location and parse reason

## 3) Module Ownership

- `main`: process-level behavior (CLI lifecycle, stderr, exit codes)
- `config`: argument parsing and runtime configuration
- `app`: orchestration and aggregation logic
- `io`: file reading and row-to-domain parsing

## 4) Parsing Boundary Rule

`app` should consume typed records, not raw strings.

- allowed in `app`: domain fields and analysis reducers
- not allowed in `app`: string splitting, CSV tokenization, raw parse branching

## 5) Performance and Memory Rule

- default to streaming ingestion
- avoid loading full file into memory unless a feature explicitly requires random access
- document any intentional materialization decision in an ADR

## 6) Error and Output Conventions

- normal program output: stdout
- warnings and errors: stderr
- non-recoverable failures: non-zero exit code

## 7) Testing Guidance

- parser tests: valid and invalid row fixtures
- app tests: typed-record fixtures and expected aggregates
- integration tests: sample file and end-to-end output behavior
