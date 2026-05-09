use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

fn get_api_url() -> String {
    env::var("AERON_CACHE_API_URL").unwrap_or_else(|_| "http://localhost:7070/api/v1/cache".to_string())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("CacheCLI")?;
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
    let mut cmd = Command::cargo_bin("CacheCLI")?;
    cmd.args(&["--api-url", &api_url, "create", cache_name]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Created cache with id: {}", cache_name)));

    // 2. Insert Item
    let mut cmd = Command::cargo_bin("CacheCLI")?;
    cmd.args(&["--api-url", &api_url, "insert", cache_name, "mykey", "myvalue"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Put item into cache with id: {}", cache_name)));

    // 3. Get Item
    let mut cmd = Command::cargo_bin("CacheCLI")?;
    cmd.args(&["--api-url", &api_url, "get", cache_name, "mykey"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Got item from cache {} on key mykey with value myvalue", cache_name)));

    // 4. Remove Item
    let mut cmd = Command::cargo_bin("CacheCLI")?;
    cmd.args(&["--api-url", &api_url, "remove", cache_name, "mykey"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Removed item from cache {} on key mykey", cache_name)));

    // 5. Delete Cache (using the --yes flag)
    let mut cmd = Command::cargo_bin("CacheCLI")?;
    cmd.args(&["--api-url", &api_url, "delete", cache_name, "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Deleted cache: {}", cache_name)));

    Ok(())
}

