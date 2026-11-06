use crate::cacheresponses::ErrorResponse;

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct PrimeNearCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) wasAlreadyWarm: bool,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum PrimeNearCacheResult {
    Ok(PrimeNearCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct StopNearCacheResponse {
    pub(crate) cacheId: String,
    pub(crate) wasCached: bool,
    pub(crate) operationStatus: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum StopNearCacheResult {
    Ok(StopNearCacheResponse),
    Err(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct NearCacheDetails {
    pub(crate) cacheId: String,
    pub(crate) itemCount: i64,
}
