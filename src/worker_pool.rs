//! Parallel mutant execution worker pool.

use crate::mutant::Mutant;
use crate::result::MutantResult;
use crate::runner::{run_mutant, RunnerConfig};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;

/// A pool of workers that executes mutant test runs in parallel.
pub struct WorkerPool {
    pool: Arc<ThreadPool>,
}

impl WorkerPool {
    /// Creates a worker pool with the given number of worker threads.
    pub fn new(workers: usize) -> Result<Self, String> {
        let pool = ThreadPoolBuilder::new()
            .num_threads(workers.max(1))
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Returns the recommended worker count for the current machine.
    pub fn default_workers() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }

    /// Runs all mutants in parallel, returning results in the same order as input.
    pub fn run_mutants(
        &self,
        config: &RunnerConfig,
        jobs: Vec<MutantJob>,
        progress: Option<Arc<Progress>>,
    ) -> Vec<MutantResult> {
        self.pool.install(|| {
            jobs.into_par_iter()
                .map(|job| {
                    let result = run_mutant(config, &job.mutant, &job.source);
                    if let Some(ref p) = progress {
                        p.increment();
                    }
                    result
                })
                .collect()
        })
    }

    /// Runs all mutants in parallel and streams each result through a channel
    /// as soon as it completes. The receiver can process results while the rest
    /// of the pool is still running.
    pub fn run_mutants_streaming(
        &self,
        config: &RunnerConfig,
        jobs: Vec<MutantJob>,
        progress: Option<Arc<Progress>>,
    ) -> mpsc::Receiver<MutantResult> {
        let (tx, rx) = mpsc::channel();
        let config = config.clone();
        let pool = Arc::clone(&self.pool);

        std::thread::spawn(move || {
            pool.install(|| {
                jobs.into_par_iter().for_each(|job| {
                    let result = run_mutant(&config, &job.mutant, &job.source);
                    if let Some(ref p) = progress {
                        p.increment();
                    }
                    let _ = tx.send(result);
                });
            });
        });

        rx
    }
}

/// A single mutant execution job.
#[derive(Debug, Clone)]
pub struct MutantJob {
    pub mutant: Mutant,
    pub source: String,
}

/// Thread-safe progress counter.
pub struct Progress {
    completed: AtomicUsize,
    total: usize,
}

impl Progress {
    /// Creates a progress tracker for `total` jobs.
    pub fn new(total: usize) -> Self {
        Self {
            completed: AtomicUsize::new(0),
            total,
        }
    }

    /// Increments the completed counter and prints progress.
    pub fn increment(&self) {
        let completed = self.completed.fetch_add(1, Ordering::SeqCst) + 1;
        if completed % 10 == 0 || completed == self.total {
            eprintln!(
                "  progress: {}/{} ({:.0}%)",
                completed,
                self.total,
                (completed as f64 / self.total.max(1) as f64) * 100.0
            );
        }
    }
}

use rayon::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use std::path::PathBuf;
    use std::time::Duration;

    fn temp_project_root() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "lmt-pool-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn dummy_job(id: &str) -> MutantJob {
        let project_root = temp_project_root();
        MutantJob {
            mutant: Mutant::from_candidate(
                CandidateMutant {
                    start_byte: 0,
                    end_byte: 0,
                    replacement: String::new(),
                },
                "dummy",
                project_root.join(format!("src/{id}.lua")),
                "local x = 1",
            ),
            source: "local x = 1".to_string(),
        }
    }

    fn config_for(jobs: &[MutantJob], command: Vec<String>) -> RunnerConfig {
        let project_root = jobs
            .first()
            .map(|j| {
                j.mutant
                    .file
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf()
            })
            .unwrap_or_else(temp_project_root);
        RunnerConfig {
            command,
            timeout: Duration::from_secs(1),
            project_root,
            snippet_limit: 1000,
        }
    }

    #[test]
    fn runs_mutants_in_parallel() {
        let pool = WorkerPool::new(2).unwrap();
        let jobs = vec![dummy_job("a"), dummy_job("b"), dummy_job("c")];
        let config = config_for(
            &jobs,
            vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
        );
        let results = pool.run_mutants(&config, jobs, None);
        assert_eq!(results.len(), 3);
        assert!(results
            .iter()
            .all(|r| matches!(r, MutantResult::Survived { .. })));
    }

    #[test]
    fn default_workers_is_non_zero() {
        assert!(WorkerPool::default_workers() >= 1);
    }

    #[test]
    fn results_are_deterministic() {
        let pool = WorkerPool::new(2).unwrap();
        let jobs = vec![dummy_job("a"), dummy_job("b")];
        let config = config_for(
            &jobs,
            vec!["sh".to_string(), "-c".to_string(), "exit 1".to_string()],
        );
        let first: Vec<_> = pool
            .run_mutants(&config, jobs.clone(), None)
            .into_iter()
            .map(|r| r.category().to_string())
            .collect();
        let second: Vec<_> = pool
            .run_mutants(&config, jobs, None)
            .into_iter()
            .map(|r| r.category().to_string())
            .collect();
        assert_eq!(first, second);
    }
}
