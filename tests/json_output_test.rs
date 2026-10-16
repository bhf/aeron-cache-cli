use assert_cmd::Command;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

/// Starts a one-shot HTTP server that answers the first request with `body`
/// and returns the API base URL to point the CLI at.
fn mock_server(body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });
    format!("http://{}/api/v1", addr)
}

fn run_json(body: &'static str, args: &[&str]) -> (String, Value) {
    let url = mock_server(body);
    let mut cmd = Command::cargo_bin("ac-cli").unwrap();
    cmd.args(["--api-url", &url]).args(args);
    let out = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8(out).unwrap();
    let value = serde_json::from_str(&text).expect("stdout should be valid JSON");
    (text, value)
}

#[test]
fn json_get_item() {
    let body = r#"{"cacheId":"c","key":"k","value":"v","operationStatus":"OK"}"#;
    let (_, v) = run_json(body, &["--json", "get", "c", "k"]);
    assert_eq!(v, json!({"cacheId":"c","key":"k","value":"v","operationStatus":"OK"}));
}

#[test]
fn json_flag_after_subcommand() {
    let body = r#"{"cacheId":"c","operationStatus":"OK"}"#;
    let (_, v) = run_json(body, &["create", "c", "--json"]);
    assert_eq!(v["cacheId"], "c");
}

#[test]
fn json_error_response_is_passed_through() {
    let body = r#"{"errorMsg":"Cache not found","helpMsg":"Create it first"}"#;
    let (text, v) = run_json(body, &["--json", "get-cache", "missing"]);
    assert_eq!(v["errorMsg"], "Cache not found");
    assert!(!text.contains("Error:"));
}

#[test]
fn json_list_caches_is_array() {
    let body = r#"[{"cacheId":"a","itemCount":1},{"cacheId":"b","itemCount":2}]"#;
    let (_, v) = run_json(body, &["--json", "list-caches"]);
    assert_eq!(v.as_array().unwrap().len(), 2);
}

#[test]
fn json_stats() {
    let body = r#"{"totalOpsCount":5,"totalCachesCount":2,"totalItemsCount":3,"errorCount":0}"#;
    let (_, v) = run_json(body, &["--json", "stats"]);
    assert_eq!(v["totalOpsCount"], 5);
}

#[test]
fn json_counter_get() {
    let body = r#"{"cacheId":"cc","key":"k","value":7,"operationStatus":"OK"}"#;
    let (_, v) = run_json(body, &["--json", "get-counter", "cc", "k"]);
    assert_eq!(v["value"], 7);
}

#[test]
fn json_unparseable_body_is_not_an_error() {
    // JSON mode must not depend on the response matching our structs.
    let (_, v) = run_json(r#"{"anything":"goes"}"#, &["--json", "create", "c"]);
    assert_eq!(v["anything"], "goes");
}

#[test]
fn without_flag_output_is_human_readable() {
    let url = mock_server(r#"{"cacheId":"c","operationStatus":"OK"}"#);
    Command::cargo_bin("ac-cli")
        .unwrap()
        .args(["--api-url", &url, "create", "c"])
        .assert()
        .success()
        .stdout("Created cache with id: c\n");
}

#[test]
fn json_delete_with_yes() {
    let body = r#"{"cacheId":"c","operationStatus":"OK"}"#;
    let (_, v) = run_json(body, &["--json", "delete", "c", "--yes"]);
    assert_eq!(v["cacheId"], "c");
}
