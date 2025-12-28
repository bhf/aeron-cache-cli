use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct CreateRequest<'a> {
    pub(crate) cacheId: &'a str,
}

#[derive(Serialize)]
pub(crate) struct PutItemRequest<'a> {
    pub(crate) cacheId: &'a str,
    pub(crate) key: &'a str,
    pub(crate) value: &'a str,
}