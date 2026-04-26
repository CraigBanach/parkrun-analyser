# parkrun-analyser

Analyse your parkrun history from the command line: PBs, trends, venues, streaks, and more.

## Why this exists

This project is a small Rust CLI for exploring parkrun results data in a way that is fast, scriptable, and easy to extend.

The aim is to build something genuinely useful while learning more about:

- Rust
- CLI design
- parsing and analysing structured data
- building tools with a tight, focused scope

## Planned features

The first version will focus on a simple CSV input and a few useful commands.

### Summary

Show a high-level overview of a runner's history, such as:

- total parkruns
- first and latest parkrun
- PB
- average and median time
- number of PBs
- current streak and longest streak
- most visited venue

### Venues

Break down performance by venue, for example:

- number of visits
- fastest venue
- most visited venue
- average time by venue

### Trend analysis

Surface changes over time, such as:

- rolling average over recent runs
- PB progression
- recent form vs lifetime average
- longest gaps between runs

### Comparisons

Potential future support for:

- comparing two runners
- comparing years or time periods
- simple performance projections

## Early command ideas

```bash
parkrun-analyser summary results.csv
parkrun-analyser venues results.csv
parkrun-analyser trend results.csv
```

## Input

The project will start with a simple CSV format so the first version can stay focused and shippable.

A likely shape is something like:

```csv
date,event,position,time_seconds,age_grade
2024-01-06,Dunfermline,112,1679,54.3
2024-01-13,Dunfermline,98,1654,55.1
```

The exact format may evolve once the first parser is implemented.

## Project goals

This project is intended to be:

- small enough to finish
- useful enough to care about
- rich enough to teach real things

It is not intended to be a full training platform or a clone of any existing fitness app.

## Tech

- Rust
- Cargo
- CSV parsing
- lightweight CLI ergonomics

## Testing

Run the full test suite:

```bash
cargo test
```

Run only the integration tests for the current `io` streaming behavior:

```bash
cargo test --test io_integration
```

## Development principles

- keep the scope tight
- favour clear output over cleverness
- ship a working v1 before adding fancy features
- prefer boring, maintainable choices

## Rough roadmap

### v0.1

- parse CSV input
- print a basic summary
- support venue breakdowns

### v0.2

- add trend analysis
- improve terminal output formatting
- add tests for parsing and core stats

### v0.3

- compare two datasets or runners
- support additional input formats if needed

## Contributing

This is currently a personal learning project, but ideas, issues, and improvements are welcome.

## Licence

MIT
