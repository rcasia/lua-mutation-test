//! Incremental mutation testing: caching, change detection, and watch mode.

pub mod cache;
pub mod change_detection;
pub mod watch;

use crate::config::Config;
use crate::mutant::Mutant;
use crate::result::MutantResult;
use crate::runner::RunnerConfig;
use crate::worker_pool::{MutantJob, Progress, WorkerPool};
use cache::{result_from_entry, CacheEntry, CacheFile};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Result of an incremental mutation run.
#[derive(Debug, Clone)]
pub struct IncrementalRunResult {
    /// Combined results from cache and new executions.
    pub results: Vec<MutantResult>,
    /// Number of mutants executed (not served from cache).
    pub ran: usize,
    /// Number of mutants served from cache.
    pub cached: usize,
}

/// Runs mutation tests incrementally, reusing cached results for unchanged files
/// and mutants.
pub fn run_incremental(
    project_root: &Path,
    config: &Config,
    runner_config: &RunnerConfig,
    source_files: &[PathBuf],
    mutants: Vec<(Mutant, String)>,
    workers: usize,
) -> Result<IncrementalRunResult, String> {
    eprintln!("Loading cache...");
    let config_hash = cache::config_hash(config);
    let mut cache = CacheFile::load(project_root);

    // Invalidate the entire cache when the configuration changes.
    if cache.config_hash != config_hash {
        eprintln!("  configuration changed, invalidating cache");
        cache.entries.clear();
        cache.file_hashes.clear();
    }

    // Determine which source files changed since the last run.
    let changed_files = change_detection::changed_files(
        project_root,
        &cache.file_hashes,
        source_files,
        cache.git_head.as_deref(),
    )?;
    let changed_set: std::collections::HashSet<PathBuf> =
        changed_files.iter().map(PathBuf::from).collect();

    // Drop cached entries for changed files.
    cache.entries.retain(|_, entry| {
        let entry_path = PathBuf::from(&entry.file);
        !changed_set.contains(&entry_path)
    });

    // Partition mutants into cached hits and jobs that need execution.
    let mut cached_results = Vec::new();
    let mut jobs = Vec::new();
    for (mutant, source) in mutants {
        let key = CacheFile::key(&mutant.file, &mutant.id, &config_hash);
        if let Some(entry) = cache.entries.get(&key) {
            cached_results.push(result_from_entry(mutant, entry));
        } else {
            jobs.push(MutantJob { mutant, source });
        }
    }

    let ran = jobs.len();
    let cached = cached_results.len();
    eprintln!("  cache: {} hit(s), {} mutant(s) to run", cached, ran);

    // Execute mutants that were not cacheable.
    let pool = WorkerPool::new(workers)?;
    eprintln!("Running mutants with {} worker(s)...", workers);
    let progress = Arc::new(Progress::new(ran.max(1)));
    let new_results = pool.run_mutants(runner_config, jobs, Some(progress));
    eprintln!("  mutant run complete");

    // Update the cache with newly computed results and current file state.
    for result in &new_results {
        let mutant = result.mutant();
        let key = CacheFile::key(&mutant.file, &mutant.id, &config_hash);
        cache
            .entries
            .insert(key, CacheEntry::from_result(result, &config_hash));
    }
    cache.file_hashes = cache::compute_file_hashes(source_files);
    cache.config_hash = config_hash;
    cache.git_head = current_git_head(project_root);
    eprintln!("Saving cache...");
    cache.save(project_root)?;

    // Merge cached and new results, preserving input order as much as possible.
    let mut results = cached_results;
    results.extend(new_results);

    Ok(IncrementalRunResult {
        results,
        ran,
        cached,
    })
}

/// Returns the current git HEAD commit hash, if available.
fn current_git_head(project_root: &Path) -> Option<String> {
    use std::process::Command;
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(project_root)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use crate::runner::RunnerConfig;
    use std::path::PathBuf;
    use std::time::Duration;

    fn dummy_mutant(file: &str) -> (Mutant, String) {
        let source = "local x = 1";
        let mutant = Mutant::from_candidate(
            CandidateMutant {
                start_byte: 0,
                end_byte: 0,
                replacement: String::new(),
            },
            "dummy",
            PathBuf::from(file),
            source,
        );
        (mutant, source.to_string())
    }

    fn temp_dir() -> PathBuf {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("lmt-inc-test-{}-{}", std::process::id(), n));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn incremental_run_uses_cache_for_unchanged_files() {
        let dir = temp_dir();

        let file = dir.join("foo.lua");
        std::fs::write(&file, "local x = 1").unwrap();

        let config = Config::default().with_test_command("exit 0");
        let runner_config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
            timeout: Duration::from_secs(1),
            project_root: dir.clone(),
            snippet_limit: 100,
        };

        // First run: executes the mutant.
        let mutants = vec![dummy_mutant(file.to_str().unwrap())];
        let first = run_incremental(
            &dir,
            &config,
            &runner_config,
            &[file.clone()],
            mutants.clone(),
            1,
        )
        .unwrap();
        assert_eq!(first.ran, 1);
        assert_eq!(first.cached, 0);

        // Second run: cache hit.
        let second =
            run_incremental(&dir, &config, &runner_config, &[file.clone()], mutants, 1).unwrap();
        assert_eq!(second.ran, 0);
        assert_eq!(second.cached, 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn incremental_run_invalidates_cache_when_file_changes() {
        let dir = temp_dir();

        let file = dir.join("foo.lua");
        std::fs::write(&file, "local x = 1").unwrap();

        let config = Config::default().with_test_command("exit 0");
        let runner_config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
            timeout: Duration::from_secs(1),
            project_root: dir.clone(),
            snippet_limit: 100,
        };

        let mutants = vec![dummy_mutant(file.to_str().unwrap())];
        let _ = run_incremental(
            &dir,
            &config,
            &runner_config,
            &[file.clone()],
            mutants.clone(),
            1,
        )
        .unwrap();

        // Modify the source file.
        std::fs::write(&file, "local x = 2").unwrap();

        let second =
            run_incremental(&dir, &config, &runner_config, &[file.clone()], mutants, 1).unwrap();
        assert_eq!(second.ran, 1);
        assert_eq!(second.cached, 0);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn incremental_run_invalidates_cache_when_config_changes() {
        let dir = temp_dir();

        let file = dir.join("foo.lua");
        std::fs::write(&file, "local x = 1").unwrap();

        let config1 = Config::default().with_test_command("exit 0");
        let config2 = Config::default()
            .with_test_command("exit 0")
            .with_framework("luaunit");
        let runner_config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
            timeout: Duration::from_secs(1),
            project_root: dir.clone(),
            snippet_limit: 100,
        };

        let mutants = vec![dummy_mutant(file.to_str().unwrap())];
        let _ = run_incremental(
            &dir,
            &config1,
            &runner_config,
            &[file.clone()],
            mutants.clone(),
            1,
        )
        .unwrap();

        let second =
            run_incremental(&dir, &config2, &runner_config, &[file.clone()], mutants, 1).unwrap();
        assert_eq!(second.ran, 1);
        assert_eq!(second.cached, 0);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
