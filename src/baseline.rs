use crate::adapter::{Adapter, TestResult};
use std::path::Path;

/// Result of running the test suite against the original, unmutated source.
#[derive(Debug, Clone, PartialEq)]
pub struct Baseline {
    /// Outcome of the baseline test run.
    pub result: TestResult,
}

/// Runs the discovered test files against the original source using the given adapter.
pub fn run_baseline(adapter: &dyn Adapter, test_files: &[&Path]) -> Baseline {
    let result = adapter.run(test_files);
    Baseline { result }
}

impl Baseline {
    /// Returns true when the baseline test run succeeded.
    pub fn passed(&self) -> bool {
        self.result.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysPass;

    impl Adapter for AlwaysPass {
        fn run(&self, _test_files: &[&Path]) -> TestResult {
            TestResult::new(true, "ok", "")
        }
    }

    struct AlwaysFail;

    impl Adapter for AlwaysFail {
        fn run(&self, _test_files: &[&Path]) -> TestResult {
            TestResult::new(false, "", "failure")
        }
    }

    #[test]
    fn baseline_records_passing_result() {
        let files: Vec<&Path> = vec![];
        let baseline = run_baseline(&AlwaysPass, &files);
        assert!(baseline.passed());
        assert!(baseline.result.stdout.contains("ok"));
    }

    #[test]
    fn baseline_records_failing_result() {
        let files: Vec<&Path> = vec![];
        let baseline = run_baseline(&AlwaysFail, &files);
        assert!(!baseline.passed());
        assert!(baseline.result.stderr.contains("failure"));
    }
}
