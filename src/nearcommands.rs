use crate::nearresponses::{NearCacheDetails, PrimeNearCacheResult, StopNearCacheResult};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;

/// Prime a near cache for `cache_name` so its local copy is warmed up front.
pub(crate) fn process_prime_near_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/cache/{}", aeron_cache_api_url, cache_name);
    prime_near_cache(rest_client, url, "cache")
}

/// Prime a near counter cache for `cache_name` so its local copy is warmed up front.
pub(crate) fn process_prime_near_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/counters/{}", aeron_cache_api_url, cache_name);
    prime_near_cache(rest_client, url, "counter cache")
}

/// Stop near caching for `cache_name`: drop the subscription and remove the local copy.
pub(crate) fn process_stop_near_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/cache/{}", aeron_cache_api_url, cache_name);
    stop_near_cache(rest_client, url, "cache")
}

/// Stop near caching for counter `cache_name`: drop the subscription and remove the local copy.
pub(crate) fn process_stop_near_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/counters/{}", aeron_cache_api_url, cache_name);
    stop_near_cache(rest_client, url, "counter cache")
}

/// List the caches currently held warm in the near cache, with their local item counts.
pub(crate) fn process_list_near_caches(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/caches", aeron_cache_api_url);
    list_near_caches(rest_client, url, "near cache")
}

/// List the counter caches currently held warm in the near cache, with their local item counts.
pub(crate) fn process_list_near_counter_caches(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/near/counters-caches", aeron_cache_api_url);
    list_near_caches(rest_client, url, "near counter cache")
}

fn prime_near_cache(
    rest_client: &Client,
    url: &str,
    noun: &str,
) -> Result<(), Box<dyn Error>> {
    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    if near_disabled_hint(response.status()) {
        return Ok(());
    }

    let body = response.text()?;

    if crate::output::print_json_if_enabled(&body) {
        return Ok(());
    }
    let result: PrimeNearCacheResult = serde_json::from_str(&body)?;

    match result {
        PrimeNearCacheResult::Ok(resp) => {
            if resp.wasAlreadyWarm {
                println!("Near {} {} was already warm", noun, resp.cacheId);
            } else {
                println!("Primed near {} {}", noun, resp.cacheId);
            }
        }
        PrimeNearCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

fn stop_near_cache(
    rest_client: &Client,
    url: &str,
    noun: &str,
) -> Result<(), Box<dyn Error>> {
    let response = rest_client.delete(url).send()?;

    if near_disabled_hint(response.status()) {
        return Ok(());
    }

    let body = response.text()?;

    if crate::output::print_json_if_enabled(&body) {
        return Ok(());
    }
    let result: StopNearCacheResult = serde_json::from_str(&body)?;

    match result {
        StopNearCacheResult::Ok(resp) => {
            if resp.wasCached {
                println!("Stopped near caching {} {}", noun, resp.cacheId);
            } else {
                println!("Near {} {} was not being cached", noun, resp.cacheId);
            }
        }
        StopNearCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

fn list_near_caches(
    rest_client: &Client,
    url: &str,
    noun: &str,
) -> Result<(), Box<dyn Error>> {
    let response = rest_client.get(url).send()?;

    if near_disabled_hint(response.status()) {
        return Ok(());
    }

    let body = response.text()?;
    if crate::output::print_json_if_enabled(&body) {
        return Ok(());
    }

    if let Ok(caches) = serde_json::from_str::<Vec<NearCacheDetails>>(&body) {
        if caches.is_empty() {
            println!("No warm {}s found.", noun);
        } else {
            for cache in caches {
                println!("Cache ID: {}, Items: {}", cache.cacheId, cache.itemCount);
            }
        }
    } else {
        println!("Error parsing response: {}", body);
    }
    Ok(())
}

/// The near cache management routes are only registered when the backend is
/// started with `NEAR_CACHE_ENABLED=true`; otherwise they return 404. Detect
/// that and print a helpful hint instead of letting a JSON parse error bubble up.
fn near_disabled_hint(status: StatusCode) -> bool {
    if status == StatusCode::NOT_FOUND {
        if crate::output::is_json() {
            println!(
                "{}",
                serde_json::json!({
                    "errorMsg": "Near cache endpoints are not available",
                    "helpMsg": "Start the Aeron Cache backend with NEAR_CACHE_ENABLED=true to enable near caching"
                })
            );
        } else {
            println!("Near cache endpoints are not available.");
            println!(
                "Help: Start the Aeron Cache backend with NEAR_CACHE_ENABLED=true to enable near caching."
            );
        }
        true
    } else {
        false
    }
}
