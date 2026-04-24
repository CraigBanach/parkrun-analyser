# Milestone 1 PRD: Basic parkrun Summary CLI

## Status

Planned

## Goal

Build the first useful version of `parkrun-analyser`: a CLI command that reads a parkrun results CSV and prints a basic summary.

This milestone is deliberately small. The aim is to prove the core loop:

```text
read CSV -> parse results -> compute simple stats -> print useful output
```

## Why this milestone matters

Before adding venue analysis, trend analysis, PB progression, Garmin/Strava imports, or richer output, the project needs a working foundation.

This first milestone should answer:

- Can the tool read the canonical CSV format?
- Can it model parkrun results cleanly?
- Can it compute useful beginner-level stats?
- Can it be run from the command line with a real sample dataset?

## User story

As a runner with a parkrun results history CSV, I want to run one command and see a concise summary of my parkrun history, so that I can quickly understand my total runs, PB, average performance, and most visited venue.

## Input

The command should accept a CSV file path.

Example:

```bash
parkrun-analyser summary samples/parkrun-results.csv
```

The sample CSV already exists at:

```text
samples/parkrun-results.csv
```

Expected columns:

```csv
date,event,run_number,position,time,age_grade,pb
2026-04-11,Lochore Meadows,307,118,37:59,34.36,false
```

## Output

The command should print a readable text summary.

Example shape:

```text
parkrun summary
---------------
Total runs: 37
First run: 2025-03-15 at Dunfermline
Latest run: 2026-04-11 at Lochore Meadows
PB: 29:51 at Lochore Meadows on 2025-06-14
Average time: 35:48
Best age grade: 43.55%
Most visited venue: Lochore Meadows (30 runs)
```

Exact formatting can change, but the output should remain simple and human-readable.

## Functional requirements

### CLI

- Provide a `summary` command.
- Accept a required CSV file path argument.
- Print an error if the file path is missing.
- Print an error if the file cannot be read.

### CSV parsing

- Parse the following fields:
  - `date`
  - `event`
  - `run_number`
  - `position`
  - `time`
  - `age_grade`
  - `pb`
- Convert `date` into a date type or a consistently comparable representation.
- Convert `time` from `mm:ss` into seconds internally.
- Convert `age_grade` into a numeric value.
- Convert `pb` into a boolean.

### Summary stats

Compute and display:

- total number of runs
- first run date and venue
- latest run date and venue
- PB time, venue, and date
- average time
- best age grade
- most visited venue and visit count

## Non-goals

Do not include these in milestone 1:

- scraping parkrun webpages
- parsing saved HTML
- Garmin or Strava integration
- charts or graphs
- multi-runner comparisons
- predictions
- complex terminal UI
- database storage
- web app
- config files

## Suggested implementation shape

Keep the implementation small and direct.

Possible files:

```text
src/main.rs
src/models.rs
src/parser.rs
src/stats.rs
```

Suggested responsibilities:

- `main.rs` wires CLI args to behaviour
- `models.rs` defines `ParkrunResult`
- `parser.rs` reads/parses CSV
- `stats.rs` computes summary values

This is not a hard requirement. Prefer clear, working code over architecture ceremony.

## Data model sketch

```rust
struct ParkrunResult {
    date: NaiveDate,
    event: String,
    run_number: u32,
    position: u32,
    time_seconds: u32,
    age_grade: f32,
    pb: bool,
}
```

A separate struct may be useful for computed output:

```rust
struct SummaryStats {
    total_runs: usize,
    first_run: ParkrunResult,
    latest_run: ParkrunResult,
    pb_run: ParkrunResult,
    average_time_seconds: u32,
    best_age_grade: f32,
    most_visited_venue: String,
    most_visited_venue_count: usize,
}
```

Use ownership/cloning pragmatically for this milestone. The goal is to learn and ship, not design the perfect abstraction.

## Edge cases for milestone 1

Handle these reasonably:

- empty CSV
- malformed time value
- missing required column
- invalid date
- invalid numeric values

It is acceptable for the first version to return a clear error and exit rather than recover.

## Acceptance criteria

This milestone is complete when:

- `cargo run -- summary samples/parkrun-results.csv` runs successfully
- the command prints all required summary stats
- malformed input produces a readable error
- core parsing and stat calculation logic has at least a few tests
- `cargo fmt` passes
- `cargo clippy` passes without obvious warnings
- `cargo test` passes

## Test ideas

Add tests for:

- parsing `29:51` into seconds
- formatting seconds back into `mm:ss`
- finding the PB from multiple results
- computing average time
- finding most visited venue
- handling an empty result list

## Future milestones

After this milestone, likely next steps are:

1. Venue breakdown command
2. PB progression command
3. Recent form / rolling average command
4. Saved parkrun HTML importer
5. Garmin/Strava integration experiments

## Principle

Keep this milestone boring and finished.

The win is not fancy output. The win is a working Rust CLI that reads real data and returns useful analysis.
