use crate::cacherequests::{CreateRequest, PutItemRequest};
use crate::cacheresponses::{
    CreateCacheResult, DeleteCacheResult, DeleteItemResult, GetItemResult, PutItemResult,
};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;

pub(crate) fn process_get_item(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
) -> Result<(), Box<dyn Error>> {

    let url = &format!(
        "{}/cache/{}/{}",
        aeron_cache_api_url, cache_name, key
    );

    let get_item_response = rest_client
        .get(url)
        .send()?;

    let body = get_item_response.text()?;
    let result: GetItemResult = serde_json::from_str(&body)?;

    match result {
        GetItemResult::Ok(resp) => println!(
            "Got item from cache {} on key {} with value {}",
            resp.cacheId, resp.key, resp.value
        ),
        GetItemResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_remove_item(
    rest_client: Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
) -> Result<(), Box<dyn Error>> {

    let url = &format!(
        "{}/cache/{}/{}",
        aeron_cache_api_url, cache_name, key
    );

    let remove_item_response = rest_client
        .delete(url)
        .send()?;

    let body = remove_item_response.text()?;
    let result: DeleteItemResult = serde_json::from_str(&body)?;

    match result {
        DeleteItemResult::Ok(resp) => println!(
            "Removed item from cache {} on key {}",
            resp.cacheId, resp.key
        ),
        DeleteItemResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_delete_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let delete_cache_response = rest_client
        .delete(&format!("{}/cache/{}", aeron_cache_api_url, cache_name))
        .send()?;

    let body = delete_cache_response.text()?;
    let result: DeleteCacheResult = serde_json::from_str(&body)?;

    match result {
        DeleteCacheResult::Ok(resp) => println!("Deleted cache: {}", resp.cacheId),
        DeleteCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_insert_item(
    rest_client: &Client,
    aeron_cache_api_url: &String,
    cache_name: &String,
    key: &String,
    value: String,
) -> Result<(), Box<dyn Error>> {
    let put_item_request = PutItemRequest {
        key,
        value: &value,
    };

    let url = &format!("{}/cache/{}", aeron_cache_api_url, cache_name);

    let put_item_response = rest_client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&put_item_request)?)
        .send()?;

    let body = put_item_response.text()?;
    let result: PutItemResult = serde_json::from_str(&body)?;

    match result {
        PutItemResult::Ok(resp) => println!("Put item into cache with id: {}", resp.cacheId),
        PutItemResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_create_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    name: &String,
) -> Result<(), Box<dyn Error>> {
    let create_cache_request = CreateRequest { cacheId: name };

    let formatted_url = &format!("{}/cache", aeron_cache_api_url);

    let create_cache_response = rest_client
        .post(formatted_url)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&create_cache_request)?)
        .send()?;

    let body = create_cache_response.text()?;
    let result: CreateCacheResult = serde_json::from_str(&body)?;

    match result {
        CreateCacheResult::Ok(resp) => println!("Created cache with id: {}", resp.cacheId),
        CreateCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }

    Ok(())
}

pub(crate) fn process_get_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/cache/{}", aeron_cache_api_url, cache_name);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;
    let result: crate::cacheresponses::GetCacheResult = serde_json::from_str(&body)?;

    match result {
        crate::cacheresponses::GetCacheResult::Ok(resp) => {
            println!("Cache ID: {}", resp.cacheId);
            println!("Status: {}", resp.status);
            if let Some(items) = resp.items {
                if items.is_empty() {
                    println!("No items in cache.");
                } else {
                    for item in items {
                        println!("Key: {}, Value: {}", item.key, item.value);
                    }
                }
            } else {
                println!("No items in cache.");
            }
        }
        crate::cacheresponses::GetCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }
    Ok(())
}

pub(crate) fn process_clear_cache(
    rest_client: &Client,
    aeron_cache_api_url: &str,
    cache_name: &String,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/cache/{}", aeron_cache_api_url, cache_name);
    let response = rest_client.patch(url).send()?;
    let body = response.text()?;
    let result: crate::cacheresponses::ClearCacheResult = serde_json::from_str(&body)?;

    match result {
        crate::cacheresponses::ClearCacheResult::Ok(resp) => {
            println!("Cleared cache: {}", resp.cacheId);
        }
        crate::cacheresponses::ClearCacheResult::Err(err) => {
            println!("Error: {}", err.errorMsg);
            println!("Help: {}", err.helpMsg);
        }
    }
    Ok(())
}

pub(crate) fn process_list_caches(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/caches", aeron_cache_api_url);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;
    
    // According to openapi.yaml, GET /api/v1/caches returns an array of CacheDetails directly.
    // If it's an error, it might not be ErrorResponse? The spec doesn't list 400 for GetCaches, just 200.
    // We can try to parse as array or ErrorResponse. Let's just parse as array.
    if let Ok(caches) = serde_json::from_str::<Vec<crate::cacheresponses::CacheDetails>>(&body) {
        if caches.is_empty() {
            println!("No caches found.");
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

pub(crate) fn process_get_stats(
    rest_client: &Client,
    aeron_cache_api_url: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &format!("{}/stats", aeron_cache_api_url);
    let response = rest_client.get(url).send()?;
    let body = response.text()?;
    if let Ok(stats) = serde_json::from_str::<crate::cacheresponses::CacheStatsResponse>(&body) {
        println!("Cache Statistics:");
        println!("Total Ops: {}", stats.totalOps);
        println!("Total Caches: {}", stats.totalCaches);
        println!("Total Items: {}", stats.totalItems);
        println!("Error Count: {}", stats.errorCount);
    } else {
        println!("Error parsing response: {}", body);
    }
    Ok(())
}
