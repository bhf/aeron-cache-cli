use std::sync::atomic::{AtomicBool, Ordering};

static JSON_OUTPUT: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_json(enabled: bool) {
    JSON_OUTPUT.store(enabled, Ordering::Relaxed);
}

/// In JSON mode, prints the raw API response body and returns true so the
/// caller can skip human-readable formatting.
pub(crate) fn print_json_if_enabled(body: &str) -> bool {
    if JSON_OUTPUT.load(Ordering::Relaxed) {
        println!("{}", body.trim());
        true
    } else {
        false
    }
}

pub(crate) fn is_json() -> bool {
    JSON_OUTPUT.load(Ordering::Relaxed)
}
