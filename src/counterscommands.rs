use crate::countersrequests::{
    CreateCounterRequest, DecrementCounterRequest, IncrementCounterRequest, PutCounterRequest,
    PutTimedCounterRequest, SetCounterRequest,
};
use crate::countersresponses::{
    CancelCounterRemovalResult, ClearCounterCacheResult, CreateCounterResult,
    DecrementCounterResult, DeleteCounterCacheResult, DeleteCounterResult, GetCounterResult,
    GetCountersResult, IncrementCounterResult, PutCounterResult, PutTimedCounterResult,
    SetCounterResult,
};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;

pub(crate) fn process_create_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    name: &String,
) -> Result<(), Box<dyn Error>> {
    let create_counter_request = CreateCounterRequest { cacheId: name };

    let url = &format!("{}/counters/", aeron_cache_api_url);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&create_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: CreateCounterResult = serde_json::from_str(&body)?;

    match result {
        CreateCounterResult::Ok(resp) => {
            println!("Created counter cache with id: {}", resp.cacheId)
        }
        CreateCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_put_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    value: i64,
) -> Result<(), Box<dyn Error>> {
    let put_counter_request = PutCounterRequest { key, value };

    let url = &format!("{}/counters/{}", aeron_cache_api_url, cache_name);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&put_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: PutCounterResult = serde_json::from_str(&body)?;

    match result {
        PutCounterResult::Ok(resp) => println!(
            "Put counter into cache {} on key {}",
            resp.cacheId, resp.key
        ),
        PutCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_put_timed_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    value: i64,
    ttl: i64,
) -> Result<(), Box<dyn Error>> {
    let put_timed_counter_request = PutTimedCounterRequest { key, value, ttl };

    let url = &format!("{}/counters/timed/{}", aeron_cache_api_url, cache_name);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&put_timed_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: PutTimedCounterResult = serde_json::from_str(&body)?;

    match result {
        PutTimedCounterResult::Ok(resp) => println!(
            "Put timed counter into cache {} on key {} with ttl {}ms",
            resp.cacheId, resp.key, ttl
        ),
        PutTimedCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_get_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters/{}/{}", aeron_cache_api_url, cache_name, key);

    let response = rest_client.get(url).send()?;

    let body = response.text()?;
    let result: GetCounterResult = serde_json::from_str(&body)?;

    match result {
        GetCounterResult::Ok(resp) => println!(
            "Got counter from cache {} on key {} with value {}",
            resp.cacheId, resp.key, resp.value
        ),
        GetCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_delete_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters/{}/{}", aeron_cache_api_url, cache_name, key);

    let response = rest_client.delete(url).send()?;

    let body = response.text()?;
    let result: DeleteCounterResult = serde_json::from_str(&body)?;

    match result {
        DeleteCounterResult::Ok(resp) => println!(
            "Removed counter from cache {} on key {}",
            resp.cacheId, resp.key
        ),
        DeleteCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_delete_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters/{}", aeron_cache_api_url, cache_name);

    let response = rest_client.delete(url).send()?;

    let body = response.text()?;
    let result: DeleteCounterCacheResult = serde_json::from_str(&body)?;

    match result {
        DeleteCounterCacheResult::Ok(resp) => {
            println!("Deleted counter cache: {}", resp.cacheId)
        }
        DeleteCounterCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_get_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters/{}", aeron_cache_api_url, cache_name);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;
    let result: GetCountersResult = serde_json::from_str(&body)?;

    match result {
        GetCountersResult::Ok(resp) => {
            println!("Counter Cache ID: {}", resp.cacheId);
            println!("Status: {}", resp.operationStatus);
            if let Some(items) = resp.items {
                if items.is_empty() {
                    println!("No counters in cache.");
                } else {
                    for item in items {
                        println!("Key: {}, Value: {}", item.key, item.value);
                    }
                }
            } else {
                println!("No counters in cache.");
            }
        }
        GetCountersResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }
    Ok(())
}

pub(crate) fn process_clear_counter_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters/{}", aeron_cache_api_url, cache_name);
    let response = rest_client.patch(url).send()?;
    let body = response.text()?;
    let result: ClearCounterCacheResult = serde_json::from_str(&body)?;

    match result {
        ClearCounterCacheResult::Ok(resp) => {
            println!("Cleared counter cache: {}", resp.cacheId);
        }
        ClearCounterCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }
    Ok(())
}

pub(crate) fn process_increment_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    amount: i64,
) -> Result<(), Box<dyn Error>> {
    let increment_counter_request = IncrementCounterRequest { key, amount };

    let url = &format!("{}/counters/increment/{}", aeron_cache_api_url, cache_name);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&increment_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: IncrementCounterResult = serde_json::from_str(&body)?;

    match result {
        IncrementCounterResult::Ok(resp) => println!(
            "Incremented counter in cache {} on key {} to value {}",
            resp.cacheId, resp.key, resp.value
        ),
        IncrementCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_decrement_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    amount: i64,
) -> Result<(), Box<dyn Error>> {
    let decrement_counter_request = DecrementCounterRequest { key, amount };

    let url = &format!("{}/counters/decrement/{}", aeron_cache_api_url, cache_name);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&decrement_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: DecrementCounterResult = serde_json::from_str(&body)?;

    match result {
        DecrementCounterResult::Ok(resp) => println!(
            "Decremented counter in cache {} on key {} to value {}",
            resp.cacheId, resp.key, resp.value
        ),
        DecrementCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_set_counter(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    value: i64,
) -> Result<(), Box<dyn Error>> {
    let set_counter_request = SetCounterRequest { key, value };

    let url = &format!("{}/counters/set/{}", aeron_cache_api_url, cache_name);

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&set_counter_request)?)
        .send()?;

    let body = response.text()?;
    let result: SetCounterResult = serde_json::from_str(&body)?;

    match result {
        SetCounterResult::Ok(resp) => println!(
            "Set counter in cache {} on key {} to value {}",
            resp.cacheId, resp.key, resp.value
        ),
        SetCounterResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_cancel_counter_removal(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!(
        "{}/counters/{}/{}/cancel-removal",
        aeron_cache_api_url, cache_name, key
    );

    let response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    let body = response.text()?;
    let result: CancelCounterRemovalResult = serde_json::from_str(&body)?;

    match result {
        CancelCounterRemovalResult::Ok(resp) => println!(
            "Cancelled scheduled removal of counter in cache {} on key {}",
            resp.cacheId, resp.key
        ),
        CancelCounterRemovalResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_list_counter_caches(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters-caches", aeron_cache_api_url);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;

    if let Ok(caches) =
        serde_json::from_str::<Vec<crate::countersresponses::CounterCacheDetails>>(&body)
    {
        if caches.is_empty() {
            println!("No counter caches found.");
        } else {
            for cache in caches {
                println!("Counter Cache ID: {}, Items: {}", cache.cacheId, cache.itemCount);
            }
        }
    } else {
        println!("Error parsing response: {}", body);
    }
    Ok(())
}

pub(crate) fn process_get_counter_stats(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/counters-stats", aeron_cache_api_url);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;
    if let Ok(stats) = serde_json::from_str::<crate::countersresponses::CounterStatsResponse>(&body)
    {
        println!("Counter Cache Statistics:");
        println!("Total Ops: {}", stats.totalOpsCount);
        println!("Total Caches: {}", stats.totalCachesCount);
        println!("Total Items: {}", stats.totalItemsCount);
        println!("Error Count: {}", stats.errorCount);
    } else {
        println!("Error parsing response: {}", body);
    }
    Ok(())
}
