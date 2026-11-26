use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn get_api_url() -> String {
    env::var("AERON_CACHE_API_URL").unwrap_or_else(|_| "http://localhost:7070/api/v1".to_string())
}

/// Assert that a read command's output eventually contains `expected`, polling for a short while.
///
/// When the backend runs with `NEAR_CACHE_ENABLED=true`, the regular read routes are served from
/// the near cache, and near reads are *non-hydrating*: a miss kicks off priming asynchronously and
/// immediately returns an empty / `UNKNOWN_CACHE` result rather than reading through to the source.
/// So the first read after a write misses until the warming subscription catches up. Polling the
/// read (each miss re-triggers the idempotent prime) lets the local copy warm up. With near caching
/// disabled the read goes straight to source and the first attempt already matches, so this returns
/// on the first iteration.
fn assert_read_contains(api_url: &str, read_args: &[&str], expected: &str) {
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut last_stdout = String::new();
    loop {
        let mut cmd = Command::cargo_bin("ac-cli").unwrap();
        cmd.args(["--api-url", api_url]).args(read_args);
        let output = cmd.output().unwrap();
        last_stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        if output.status.success() && last_stdout.contains(expected) {
            return;
        }
        if Instant::now() >= deadline {
            panic!(
                "read {:?} never produced {:?} within timeout; last stdout:\n{}",
                read_args, expected, last_stdout
            );
        }
        sleep(Duration::from_millis(250));
    }
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI for interacting with Aeron Cache"));
    Ok(())
}

#[test]
fn test_cache_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-cache-lifecycle";

    // 1. Create Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "create", cache_name]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Created cache with id: {}", cache_name)));

    // 2. Insert Item
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "insert", cache_name, "mykey", "myvalue"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Put item into cache with id: {}", cache_name)));

    // 3. Get Item (poll: served from the near cache when near caching is enabled)
    assert_read_contains(
        &api_url,
        &["get", cache_name, "mykey"],
        &format!("Got item from cache {} on key mykey with value myvalue", cache_name),
    );

    // 4. Remove Item
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "remove", cache_name, "mykey"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Removed item from cache {} on key mykey", cache_name)));

    // 5. Delete Cache (using the --yes flag)
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete", cache_name, "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted cache: "));

    Ok(())
}


#[test]
fn test_timed_item_and_cancel_removal() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-timed-item-cancel-removal";

    // Create Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "create", cache_name]);
    cmd.assert().success();

    // Insert Timed Item
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "insert-timed", cache_name, "timedkey", "timedvalue", "60000"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Put timed item into cache {} on key timedkey", cache_name)));

    // Cancel Removal
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "cancel-removal", cache_name, "timedkey"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Cancelled scheduled removal of item in cache {} on key timedkey", cache_name)));

    // Cleanup
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete", cache_name, "--yes"]);
    cmd.assert().success();

    Ok(())
}


#[test]
fn test_extended_cache_operations() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-extended-cache-operations-cache";

    // Create Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "create", cache_name]);
    cmd.assert().success();

    // Insert Item
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "insert", cache_name, "mytestkey", "mytestvalue"]);
    cmd.assert().success();

    // Get Cache (poll: near reads are non-hydrating, so the item appears once the near copy warms)
    assert_read_contains(
        &api_url,
        &["get-cache", cache_name],
        "Key: mytestkey, Value: mytestvalue",
    );

    // Clear Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "clear-cache", cache_name]);
    cmd.assert().success().stdout(predicate::str::contains("Cleared cache:"));

    // List Caches
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "list-caches"]);
    cmd.assert().success();

    // Stats
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "stats"]);
    cmd.assert().success().stdout(predicate::str::contains("Cache Statistics:"));

    // Cleanup
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete", cache_name, "--yes"]);
    cmd.assert().success();

    Ok(())
}


#[test]
fn test_counter_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-counter-lifecycle";

    // 1. Create Counter Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "create-counter-cache", cache_name]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Created counter cache with id: {}", cache_name)));

    // 2. Put Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "put-counter", cache_name, "mycounter", "42"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Put counter into cache {} on key mycounter", cache_name)));

    // 3. Get Counter (poll: served from the near cache when near caching is enabled)
    assert_read_contains(
        &api_url,
        &["get-counter", cache_name, "mycounter"],
        &format!("Got counter from cache {} on key mycounter with value 42", cache_name),
    );

    // 4. Increment Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "increment-counter", cache_name, "mycounter", "8"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Incremented counter in cache {} on key mycounter to value 50", cache_name)));

    // 5. Decrement Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "decrement-counter", cache_name, "mycounter", "10"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Decremented counter in cache {} on key mycounter to value 40", cache_name)));

    // 6. Set Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "set-counter", cache_name, "mycounter", "100"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Set counter in cache {} on key mycounter to value 100", cache_name)));

    // 7. Delete Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete-counter", cache_name, "mycounter"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Removed counter from cache {} on key mycounter", cache_name)));

    // 8. Delete Counter Cache (using the --yes flag)
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete-counter-cache", cache_name, "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted counter cache: "));

    Ok(())
}


#[test]
fn test_extended_counter_operations() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = get_api_url();
    let cache_name = "test-extended-counter-operations-cache";

    // Create Counter Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "create-counter-cache", cache_name]);
    cmd.assert().success();

    // Put Counter
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "put-counter", cache_name, "mytestcounter", "7"]);
    cmd.assert().success();

    // Get Counter Cache (poll: near reads are non-hydrating, so the counter appears once warm)
    assert_read_contains(
        &api_url,
        &["get-counter-cache", cache_name],
        "Key: mytestcounter, Value: 7",
    );

    // Clear Counter Cache
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "clear-counter-cache", cache_name]);
    cmd.assert().success().stdout(predicate::str::contains("Cleared counter cache:"));

    // List Counter Caches
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "list-counter-caches"]);
    cmd.assert().success();

    // Counter Stats
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "counter-stats"]);
    cmd.assert().success().stdout(predicate::str::contains("Counter Cache Statistics:"));

    // Cleanup
    let mut cmd = Command::cargo_bin("ac-cli")?;
    cmd.args(&["--api-url", &api_url, "delete-counter-cache", cache_name, "--yes"]);
    cmd.assert().success();

    Ok(())
}
