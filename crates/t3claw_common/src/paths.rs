//! Shared filesystem path helpers.
//!
//! `t3claw_base_dir()` resolves the T3Claw base directory used for env
//! files, session tokens, the libsql database, and other per-instance state.
//! Override with the `T3CLAW_BASE_DIR` environment variable; defaults to
//! `~/.t3claw`.

use std::path::PathBuf;
use std::sync::LazyLock;

const T3CLAW_BASE_DIR_ENV: &str = "T3CLAW_BASE_DIR";

static T3CLAW_BASE_DIR: LazyLock<PathBuf> = LazyLock::new(compute_t3claw_base_dir);

/// Compute the T3Claw base directory from the environment.
///
/// Bypasses the `LazyLock` cache. Use this in tests that mutate
/// `T3CLAW_BASE_DIR`; production callers should use [`t3claw_base_dir`].
pub fn compute_t3claw_base_dir() -> PathBuf {
    std::env::var(T3CLAW_BASE_DIR_ENV)
        .map(PathBuf::from)
        .map(|path| {
            if path.as_os_str().is_empty() {
                default_base_dir()
            } else if !path.is_absolute() {
                eprintln!(
                    "Warning: T3CLAW_BASE_DIR is a relative path '{}', resolved against current directory",
                    path.display()
                );
                path
            } else {
                path
            }
        })
        .unwrap_or_else(|_| default_base_dir())
}

fn default_base_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".t3claw")
    } else {
        eprintln!("Warning: Could not determine home directory, using current directory");
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/tmp"))
            .join(".t3claw")
    }
}

/// Get the T3Claw base directory.
///
/// Override with `T3CLAW_BASE_DIR`. Defaults to `~/.t3claw` (or
/// `./.t3claw` if the home directory cannot be determined).
///
/// Thread-safe: the value is computed once and cached in a `LazyLock`.
pub fn t3claw_base_dir() -> PathBuf {
    T3CLAW_BASE_DIR.clone()
}
