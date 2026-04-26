# Tomorrow Next Steps

Date prepared: 2026-04-25

## Goal

Stabilize the new `main` / `config` / `app` / `io` structure with a small, low-risk sequence.

## 1) Get `io` line iteration working first

- Make `io` return a lazy line iterator from a file path (`&str`).
- Keep this pass simple: no custom row types yet.
- Ensure `app` consumes that iterator and prints lines.
- Run `cargo check` and `cargo run -- samples/parkrun-results.csv`.

Done when:

- no compiler errors
- same line-print behavior as before refactor

## 2) Clean up boundary consistency

- Confirm `config` keeps fields private and uses accessor methods.
- Confirm `run` borrows config (`&Config`) and does not consume it.
- Confirm top-level errors are printed to stderr in `main`.

Done when:

- `cargo check` is clean
- no module directly accesses private `Config` internals

## 3) Introduce tolerant malformed-row handling

- Define a row-outcome type in `io` (valid item vs warning).
- Keep warnings as data, not printed side effects.
- In `app`, count skipped rows and keep a small warning sample (target 5).
- Preserve continue-on-error behavior.

Done when:

- malformed rows do not crash the run in default mode
- warning count and sample are available to report

## 4) Move from raw strings to typed data

- Replace raw line handoff with a typed domain record from `io`.
- Keep parsing logic in `io`; keep aggregation logic in `app`.
- Keep this incremental: one record type and one parse path first.

Done when:

- `app` operates on typed fields, not raw parsing logic
- parsing responsibilities are isolated to `io`

## 5) Final reporting shape

- Keep analysis output on stdout.
- Print warning summary on stderr.
- Include: total rows, valid rows, skipped rows, sample warnings.

Done when:

- output channel split is consistent (stdout vs stderr)
- summary is understandable without reading source code

## Suggested working loop

For each step:

1. Make one small edit batch.
2. Run `cargo check`.
3. If green, run the sample file.
4. Commit once the step is complete.

## Optional stretch (only if time remains)

- Add a small fixture with malformed rows under `samples/`.
- Add one integration-style test case for tolerant mode behavior.
- Draft strict-mode CLI flag notes in docs for next milestone.
