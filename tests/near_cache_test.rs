use assert_cmd::Command;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

/// Starts a one-shot HTTP server that answers the first request with the given
/// status line and `body`, capturing that request's method and path. Returns the
/// API base URL and a receiver that yields the captured "METHOD PATH" request line.
fn mock_server_status(status_line: &'static str, body: &'static str) -> (String, mpsc::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap_or(0);
            let request = String::from_utf8_lossy(&buf[..n]);
            let request_line = request.lines().next().unwrap_or("").to_string();
            // Keep just "METHOD PATH", dropping the trailing "HTTP/1.1".
            let method_path = request_line.rsplit_once(' ').map(|(mp, _)| mp.to_string()).unwrap_or(request_line);
            let _ = tx.send(method_path);
            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                status_line,
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });
    (format!("http://{}/api/v1", addr), rx)
}

fn mock_server(body: &'static str) -> (String, mpsc::Receiver<String>) {
    mock_server_status("200 OK", body)
}

/// Runs the CLI against the mock server and returns (stdout, captured request line).
fn run(body: &'static str, args: &[&str]) -> (String, String) {
    let (url, rx) = mock_server(body);
    let mut cmd = Command::cargo_bin("ac-cli").unwrap();
    cmd.args(["--api-url", &url]).args(args);
    let out = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8(out).unwrap();
    let request_line = rx.recv().unwrap();
    (text, request_line)
}

// --- prime-near-cache ---------------------------------------------------------

#[test]
fn prime_near_cache_hits_correct_route_and_reports_primed() {
    let body = r#"{"cacheId":"c","wasAlreadyWarm":false,"operationStatus":"SUCCESS"}"#;
    let (text, req) = run(body, &["prime-near-cache", "c"]);
    assert_eq!(req, "POST /api/v1/near/cache/c");
    assert_eq!(text, "Primed near cache c\n");
}

#[test]
fn prime_near_cache_reports_already_warm() {
    let body = r#"{"cacheId":"c","wasAlreadyWarm":true,"operationStatus":"SUCCESS"}"#;
    let (text, _) = run(body, &["prime-near-cache", "c"]);
    assert_eq!(text, "Near cache c was already warm\n");
}

#[test]
fn prime_near_counter_cache_hits_counters_route() {
    let body = r#"{"cacheId":"cc","wasAlreadyWarm":false,"operationStatus":"SUCCESS"}"#;
    let (text, req) = run(body, &["prime-near-counter-cache", "cc"]);
    assert_eq!(req, "POST /api/v1/near/counters/cc");
    assert_eq!(text, "Primed near counter cache cc\n");
}

// --- stop-near-cache ----------------------------------------------------------

#[test]
fn stop_near_cache_hits_correct_route_and_reports_stopped() {
    let body = r#"{"cacheId":"c","wasCached":true,"operationStatus":"SUCCESS"}"#;
    let (text, req) = run(body, &["stop-near-cache", "c"]);
    assert_eq!(req, "DELETE /api/v1/near/cache/c");
    assert_eq!(text, "Stopped near caching cache c\n");
}

#[test]
fn stop_near_cache_reports_not_cached() {
    let body = r#"{"cacheId":"c","wasCached":false,"operationStatus":"SUCCESS"}"#;
    let (text, _) = run(body, &["stop-near-cache", "c"]);
    assert_eq!(text, "Near cache c was not being cached\n");
}

#[test]
fn stop_near_counter_cache_hits_counters_route() {
    let body = r#"{"cacheId":"cc","wasCached":true,"operationStatus":"SUCCESS"}"#;
    let (text, req) = run(body, &["stop-near-counter-cache", "cc"]);
    assert_eq!(req, "DELETE /api/v1/near/counters/cc");
    assert_eq!(text, "Stopped near caching counter cache cc\n");
}

// --- list warm near caches ----------------------------------------------------

#[test]
fn list_near_caches_hits_correct_route_and_lists_items() {
    let body = r#"[{"cacheId":"a","itemCount":1},{"cacheId":"b","itemCount":2}]"#;
    let (text, req) = run(body, &["list-near-caches"]);
    assert_eq!(req, "GET /api/v1/near/caches");
    assert!(text.contains("Cache ID: a, Items: 1"));
    assert!(text.contains("Cache ID: b, Items: 2"));
}

#[test]
fn list_near_caches_handles_empty() {
    let (text, _) = run("[]", &["list-near-caches"]);
    assert_eq!(text, "No warm near caches found.\n");
}

#[test]
fn list_near_counter_caches_hits_correct_route() {
    let body = r#"[{"cacheId":"a","itemCount":5}]"#;
    let (text, req) = run(body, &["list-near-counter-caches"]);
    assert_eq!(req, "GET /api/v1/near/counters-caches");
    assert!(text.contains("Cache ID: a, Items: 5"));
}

// --- JSON passthrough ---------------------------------------------------------

#[test]
fn json_prime_near_cache_is_passed_through() {
    let body = r#"{"cacheId":"c","wasAlreadyWarm":false,"operationStatus":"SUCCESS"}"#;
    let (text, _) = run(body, &["--json", "prime-near-cache", "c"]);
    let v: Value = serde_json::from_str(&text).expect("stdout should be valid JSON");
    assert_eq!(v, json!({"cacheId":"c","wasAlreadyWarm":false,"operationStatus":"SUCCESS"}));
}

#[test]
fn json_list_near_caches_is_array() {
    let body = r#"[{"cacheId":"a","itemCount":1}]"#;
    let (text, _) = run(body, &["--json", "list-near-caches"]);
    let v: Value = serde_json::from_str(&text).expect("stdout should be valid JSON");
    assert_eq!(v.as_array().unwrap().len(), 1);
}

#[test]
fn error_response_is_rendered_human_readable() {
    let body = r#"{"errorMsg":"Badly formed request","helpMsg":"Check all values","operationStatus":"ERROR"}"#;
    let (text, _) = run(body, &["prime-near-cache", "c"]);
    assert!(text.contains("Error: Badly formed request"));
    assert!(text.contains("Help: Check all values"));
}

// --- near caching disabled (routes not registered -> 404) ---------------------

/// Runs the CLI against a mock server returning the given status/body.
fn run_status(status_line: &'static str, body: &'static str, args: &[&str]) -> String {
    let (url, _rx) = mock_server_status(status_line, body);
    let mut cmd = Command::cargo_bin("ac-cli").unwrap();
    cmd.args(["--api-url", &url]).args(args);
    let out = cmd.assert().success().get_output().stdout.clone();
    String::from_utf8(out).unwrap()
}

#[test]
fn not_found_prints_hint_instead_of_parse_error() {
    // Javalin's default 404 body is not our JSON shape; we must not surface a parse error.
    let text = run_status("404 Not Found", "Not Found", &["prime-near-cache", "c"]);
    assert!(text.contains("Near cache endpoints are not available"));
    assert!(text.contains("NEAR_CACHE_ENABLED=true"));
    assert!(!text.contains("expected"));
}

#[test]
fn not_found_on_list_prints_hint() {
    let text = run_status("404 Not Found", "Not Found", &["list-near-caches"]);
    assert!(text.contains("Near cache endpoints are not available"));
}

#[test]
fn not_found_in_json_mode_emits_json_hint() {
    let text = run_status("404 Not Found", "Not Found", &["--json", "stop-near-cache", "c"]);
    let v: Value = serde_json::from_str(&text).expect("hint should be valid JSON in --json mode");
    assert_eq!(v["errorMsg"], "Near cache endpoints are not available");
}
