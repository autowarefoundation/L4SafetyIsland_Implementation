// Copyright 2026 The Autoware Foundation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Golden-trace comparison.
//!
//! A golden file is a rendered [`Trace`]: one line per event, plus `#`
//! comment lines and blank lines, which the comparison skips. Setting the
//! environment variable `SI_BLESS_GOLDEN=1` rewrites the file from the run
//! instead of comparing, keeping its leading comment lines; review the diff
//! before committing it.

use core::fmt;
use std::{env, fs, path::Path};

use super::Trace;

/// The environment variable that rewrites golden files.
const BLESS_VARIABLE: &str = "SI_BLESS_GOLDEN";

/// The first difference between a golden trace and a run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenMismatch {
    /// One-based index among the compared (non-comment) lines.
    pub line: usize,
    /// The golden line, or `None` when the golden trace ended first.
    pub expected: Option<String>,
    /// The run's line, or `None` when the run ended first.
    pub actual: Option<String>,
}

impl fmt::Display for GoldenMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let show = |line: &Option<String>| line.clone().unwrap_or_else(|| "<end>".to_owned());
        write!(
            f,
            "trace line {}:\n  expected: {}\n  actual:   {}",
            self.line,
            show(&self.expected),
            show(&self.actual)
        )
    }
}

/// The lines of a golden text that are compared: not blank, not a comment,
/// trailing whitespace removed.
fn compared_lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(str::trim_end)
        .filter(|line| !line.trim_start().is_empty() && !line.trim_start().starts_with('#'))
        .collect()
}

/// Compare `trace` with the golden text `golden`.
///
/// # Errors
///
/// The first line where they differ.
pub fn compare_with_golden(golden: &str, trace: &Trace) -> Result<(), GoldenMismatch> {
    let rendered = trace.render();
    let expected = compared_lines(golden);
    let actual = compared_lines(&rendered);
    let longest = expected.len().max(actual.len());
    for index in 0..longest {
        let want = expected.get(index).copied();
        let got = actual.get(index).copied();
        if want != got {
            return Err(GoldenMismatch {
                line: index + 1,
                expected: want.map(str::to_owned),
                actual: got.map(str::to_owned),
            });
        }
    }
    Ok(())
}

/// Assert that `trace` matches the golden file at `path`, or rewrite the
/// file when `SI_BLESS_GOLDEN=1`.
///
/// # Panics
///
/// When the file cannot be read or written, or the trace differs from it.
pub fn assert_golden(trace: &Trace, path: &Path) {
    let existing = fs::read_to_string(path);
    if env::var(BLESS_VARIABLE).is_ok_and(|value| value == "1") {
        let header = existing
            .as_deref()
            .map(leading_comments)
            .unwrap_or_default();
        fs::write(path, header + &trace.render())
            .unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
        return;
    }
    let golden = existing.unwrap_or_else(|error| {
        panic!(
            "cannot read {}: {error}; run with {BLESS_VARIABLE}=1 to create it",
            path.display()
        )
    });
    if let Err(mismatch) = compare_with_golden(&golden, trace) {
        panic!(
            "{} differs from the run at {mismatch}\nrun with {BLESS_VARIABLE}=1 to accept the run",
            path.display()
        );
    }
}

/// The comment lines at the top of a golden file, with their newlines.
fn leading_comments(text: &str) -> String {
    let mut header = String::new();
    for line in text.lines() {
        if !line.starts_with('#') {
            break;
        }
        header.push_str(line);
        header.push('\n');
    }
    header
}

#[cfg(test)]
mod tests {
    use super::{GoldenMismatch, compare_with_golden, leading_comments};
    use crate::harness::{Trace, millis};

    fn two_line_trace() -> Trace {
        let mut trace = Trace::new();
        trace.record(millis(0), "H7", "heartbeat counter=1");
        trace.record(millis(30), "N2", "fault HPC_LIVENESS_LOST");
        trace
    }

    #[test]
    fn matches_ignoring_comments_and_blank_lines() {
        let golden = "# hpc loss\n\n    0.000 H7 heartbeat counter=1\n   30.000 N2 fault HPC_LIVENESS_LOST  \n";
        assert_eq!(compare_with_golden(golden, &two_line_trace()), Ok(()));
    }

    #[test]
    fn reports_the_first_differing_line() {
        let golden = "    0.000 H7 heartbeat counter=1\n   40.000 N2 fault HPC_LIVENESS_LOST\n";
        assert_eq!(
            compare_with_golden(golden, &two_line_trace()),
            Err(GoldenMismatch {
                line: 2,
                expected: Some("   40.000 N2 fault HPC_LIVENESS_LOST".to_owned()),
                actual: Some("   30.000 N2 fault HPC_LIVENESS_LOST".to_owned()),
            })
        );
    }

    #[test]
    fn reports_a_missing_line() {
        let golden = "    0.000 H7 heartbeat counter=1\n";
        let mismatch = compare_with_golden(golden, &two_line_trace()).unwrap_err();
        assert_eq!(mismatch.line, 2);
        assert_eq!(mismatch.expected, None);
    }

    #[test]
    fn keeps_only_the_leading_comments() {
        assert_eq!(leading_comments("# a\n# b\n1\n# c\n"), "# a\n# b\n");
    }
}
