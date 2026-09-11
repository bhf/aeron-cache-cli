use crate::cacheresponses::ErrorResponse;

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CreateCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CreateCounterResult {
    Ok(CreateCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct PutCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum PutCounterResult {
    Ok(PutCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CounterItem {
    pub(crate) key: String,
    pub(crate) value: i64,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct GetCountersResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
    pub(crate) items: Option<Vec<CounterItem>>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum GetCountersResult {
    Ok(GetCountersResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct GetCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) value: i64,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum GetCounterResult {
    Ok(GetCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct DeleteCounterCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DeleteCounterCacheResult {
    Ok(DeleteCounterCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct DeleteCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DeleteCounterResult {
    Ok(DeleteCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ClearCounterCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum ClearCounterCacheResult {
    Ok(ClearCounterCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct IncrementCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) value: i64,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum IncrementCounterResult {
    Ok(IncrementCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct DecrementCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) value: i64,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DecrementCounterResult {
    Ok(DecrementCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct SetCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) value: i64,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum SetCounterResult {
    Ok(SetCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct PutTimedCounterResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum PutTimedCounterResult {
    Ok(PutTimedCounterResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CancelCounterRemovalResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CancelCounterRemovalResult {
    Ok(CancelCounterRemovalResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CounterCacheDetails {
    pub(crate) cacheId: String,
    pub(crate) itemCount: i64,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CounterStatsResponse {
    pub(crate) totalOpsCount: i32,
    pub(crate) totalCachesCount: i32,
    pub(crate) totalItemsCount: i32,
    pub(crate) errorCount: i32,
}
