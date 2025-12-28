use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct CreateRequest<'a> {
    pub(crate) cacheId: &'a str,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct PutItemRequest<'a> {
    pub(crate) cacheId: &'a str,
    pub(crate) key: &'a str,
    pub(crate) value: &'a str,
}