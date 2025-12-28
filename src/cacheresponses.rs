
#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorResponse {
    pub(crate) errorMsg: String,
    pub(crate) helpMsg: String,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
pub(crate) struct CreateResponse {
    pub(crate) cacheId: i64,
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
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum DeleteItemResult {
    Ok(DeleteItemResponse),
    Err(ErrorResponse),
}
