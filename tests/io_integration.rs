#[path = "../src/io.rs"]
mod io;

use std::path::PathBuf;

use io::{RowOutcome, row_outcomes};

fn sample_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("samples")
        .join(file_name)
}

#[test]
fn valid_sample_has_no_warnings() {
    let sample = sample_path("parkrun-results.csv");
    let mut records = 0usize;
    let mut warnings = 0usize;

    for outcome in row_outcomes(sample.to_str().expect("sample path should be UTF-8"))
        .expect("row_outcomes should open valid sample")
    {
        match outcome {
            RowOutcome::Record(line) => {
                records += 1;
                assert!(!line.is_empty(), "record line should not be empty");
            }
            RowOutcome::Warning(_) => warnings += 1,
        }
    }

    assert!(records > 0, "expected at least one record");
    assert_eq!(warnings, 0, "did not expect warnings for valid sample");
}

#[test]
fn utf8_warning_sample_emits_six_warnings() {
    let sample = sample_path("parkrun-results-utf8-6-warnings.csv");
    let mut records = 0usize;
    let mut warnings = 0usize;

    for outcome in row_outcomes(sample.to_str().expect("sample path should be UTF-8"))
        .expect("row_outcomes should open warning sample")
    {
        match outcome {
            RowOutcome::Record(line) => {
                records += 1;
                assert!(!line.is_empty(), "record line should not be empty");
            }
            RowOutcome::Warning(message) => {
                warnings += 1;
                assert!(!message.is_empty(), "warning message should not be empty");
            }
        }
    }

    assert_eq!(warnings, 6, "expected exactly six UTF-8 warnings");
    assert!(records > warnings, "expected interspersed valid records");
}
