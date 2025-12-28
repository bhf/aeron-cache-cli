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
    let get_item_response = rest_client
        .get(&format!(
            "{}/cache/{}/{}",
            aeron_cache_api_url, cache_name, key
        ))
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
    let remove_item_response = rest_client
        .delete(&format!(
            "{}/cache/{}/{}",
            aeron_cache_api_url, cache_name, key
        ))
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
    aeron_cache_api_url: &str,
    cache_name: &String,
    key: &String,
    value: String,
) -> Result<(), Box<dyn Error>> {
    let put_item_request = PutItemRequest {
        cacheId: cache_name,
        key,
        value: &value,
    };

    let put_item_response = rest_client
        .post(&format!("{}/cache/{}", aeron_cache_api_url, cache_name))
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

    let create_cache_response = rest_client
        .post(&format!("{}/{}", aeron_cache_api_url, name))
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
