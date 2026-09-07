use lua_mutation_test::adapter::FrameworkAdapter;
use lua_mutation_test::baseline::run_baseline;
use lua_mutation_test::config::Config;
use lua_mutation_test::test_discovery::discover_tests;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static PATH_LOCK: Mutex<()> = Mutex::new(());

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn copy_fixture_to_temp(name: &str) -> tempfile::TempDir {
    let source = fixture_path(name);
    let temp = tempfile::tempdir().unwrap();
    copy_dir_all(&source, temp.path()).unwrap();
    temp
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn make_executable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path).unwrap().permissions();
        perms.set_mode(perms.mode() | 0o111);
        std::fs::set_permissions(path, perms).unwrap();
    }
}

fn with_path_prefix<F: FnOnce()>(prefix: &Path, f: F) {
    let _guard = PATH_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let old = std::env::var_os("PATH");
    let new = match &old {
        Some(old) => {
            let mut s = std::ffi::OsString::from(prefix);
            s.push(":");
            s.push(old);
            s
        }
        None => prefix.as_os_str().to_os_string(),
    };
    std::env::set_var("PATH", new);
    f();
    match old {
        Some(old) => std::env::set_var("PATH", old),
        None => std::env::remove_var("PATH"),
    }
}

#[test]
fn discovers_and_runs_busted_tests() {
    let temp = copy_fixture_to_temp("busted_project");
    let bin_dir = temp.path().to_path_buf();
    let script = bin_dir.join("busted");
    make_executable(&script);

    let config = Config::default();
    let globs = config.test_globs;
    let tests = discover_tests(temp.path(), &globs);
    assert_eq!(tests.len(), 1);

    with_path_prefix(&bin_dir, || {
        let adapter = FrameworkAdapter::Busted;
        let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
        let baseline = run_baseline(&adapter, &test_refs);
        assert!(baseline.passed());
        assert!(baseline.result.stdout.contains("busted"));
    });
}

#[test]
fn discovers_and_runs_luaunit_tests() {
    let temp = copy_fixture_to_temp("luaunit_project");
    let bin_dir = temp.path().to_path_buf();
    let script = bin_dir.join("lua");
    make_executable(&script);

    let config = Config::default();
    let tests = discover_tests(temp.path(), &config.test_globs);
    assert_eq!(tests.len(), 1);

    with_path_prefix(&bin_dir, || {
        let adapter = FrameworkAdapter::LuaUnit;
        let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
        let baseline = run_baseline(&adapter, &test_refs);
        assert!(baseline.passed());
        assert!(baseline.result.stdout.contains("lua"));
    });
}

#[test]
fn discovers_with_custom_glob_and_runs_generic_command() {
    let temp = copy_fixture_to_temp("generic_project");
    let runner = temp.path().join("run-tests.sh");
    make_executable(&runner);

    let config = Config::new(vec!["tests/**/*.lua".to_string()]).with_test_command(
        runner.to_str().unwrap().to_string(),
    );
    let tests = discover_tests(temp.path(), &config.test_globs);
    assert_eq!(tests.len(), 1);

    let adapter = FrameworkAdapter::from_config(
        config.framework.as_deref(),
        config.test_command.as_deref(),
    );
    let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
    let baseline = run_baseline(&adapter, &test_refs);
    assert!(baseline.passed());
    assert!(baseline.result.stdout.contains("custom runner"));
}

#[test]
fn aborts_when_baseline_fails() {
    let temp = copy_fixture_to_temp("failing_baseline_project");
    let bin_dir = temp.path().to_path_buf();
    let script = bin_dir.join("busted");
    make_executable(&script);

    let config = Config::default();
    let tests = discover_tests(temp.path(), &config.test_globs);
    assert_eq!(tests.len(), 1);

    with_path_prefix(&bin_dir, || {
        let adapter = FrameworkAdapter::Busted;
        let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
        let baseline = run_baseline(&adapter, &test_refs);
        assert!(!baseline.passed());
        assert!(baseline.result.stdout.contains("failure"));
    });
}
