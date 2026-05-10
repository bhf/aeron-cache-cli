
#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorResponse {
    pub(crate) errorMsg: String,
    pub(crate) helpMsg: String,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CreateResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CreateCacheResult {
    Ok(CreateResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct PutItemResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum PutItemResult {
    Ok(PutItemResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct DeleteCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DeleteCacheResult {
    Ok(DeleteCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct GetItemResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum GetItemResult {
    Ok(GetItemResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct DeleteItemResponse {
    pub(crate) cacheId: String,
    pub(crate) key: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DeleteItemResult {
    Ok(DeleteItemResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CacheItem {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct GetCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
    pub(crate) items: Option<Vec<CacheItem>>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum GetCacheResult {
    Ok(GetCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ClearCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum ClearCacheResult {
    Ok(ClearCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CacheDetails {
    pub(crate) cacheId: String,
    pub(crate) itemCount: i64,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CacheStatsResponse {
    pub(crate) totalOpsCount: i32,
    pub(crate) totalCachesCount: i32,
    pub(crate) totalItemsCount: i32,
    pub(crate) errorCount: i32,
}
