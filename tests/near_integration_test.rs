//! Live integration tests for the near cache management commands.
//!
//! These require the Aeron Cache backend to be started with `NEAR_CACHE_ENABLED=true`
//! (otherwise the near management routes return 404). They are marked `#[ignore]` so the
//! regular `cargo test` run skips them; the dedicated near-cache CI workflow enables near
//! caching and runs the whole suite including these via `cargo test -- --include-ignored`.

use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

fn get_api_url() -> String {
    env::var("AERON_CACHE_API_URL").unwrap_or_else(|_| "http://localhost:7070/api/v1".to_string())
}

#[test]
#[ignore = "requires backend started with NEAR_CACHE_ENABLED=true"]
fn near_cache_management_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-near-cache-lifecycle";

    // Setup: a cache with an item to warm.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "create", cache_name])
        .assert()
        .success();
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "insert", cache_name, "k", "v"])
        .assert()
        .success();

    // Prime (first time): warms the local copy. A 404 hint here means near caching is not
    // enabled on the backend, which is a misconfiguration of this workflow - fail loudly.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "prime-near-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Primed near cache {}", cache_name)));

    // Prime again: idempotent, reports the cache was already warm.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "prime-near-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Near cache {} was already warm", cache_name)));

    // List warm caches: our primed cache appears immediately.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "list-near-caches"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Cache ID: {}", cache_name)));

    // Stop: drops the local copy, reports it was cached.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "stop-near-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Stopped near caching cache {}", cache_name)));

    // Stop again: idempotent, reports it was not being cached.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "stop-near-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Near cache {} was not being cached", cache_name)));

    // Cleanup.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "delete", cache_name, "--yes"])
        .assert()
        .success();

    Ok(())
}

#[test]
#[ignore = "requires backend started with NEAR_CACHE_ENABLED=true"]
fn near_counter_cache_management_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-near-counter-cache-lifecycle";

    // Setup: a counter cache with a counter to warm.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "create-counter-cache", cache_name])
        .assert()
        .success();
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "put-counter", cache_name, "k", "1"])
        .assert()
        .success();

    // Prime (first time).
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "prime-near-counter-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Primed near counter cache {}", cache_name)));

    // Prime again: idempotent.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "prime-near-counter-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Near counter cache {} was already warm", cache_name)));

    // List warm counter caches.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "list-near-counter-caches"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Cache ID: {}", cache_name)));

    // Stop.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "stop-near-counter-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Stopped near caching counter cache {}", cache_name)));

    // Stop again: idempotent.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "stop-near-counter-cache", cache_name])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Near counter cache {} was not being cached", cache_name)));

    // Cleanup.
    Command::cargo_bin("ac-cli")?
        .args(["--api-url", &api_url, "delete-counter-cache", cache_name, "--yes"])
        .assert()
        .success();

    Ok(())
}
