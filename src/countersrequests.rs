use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct CreateCounterRequest<'a> {
    pub(crate) cacheId: &'a str,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct PutCounterRequest<'a> {
    pub(crate) key: &'a str,
    pub(crate) value: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct PutTimedCounterRequest<'a> {
    pub(crate) key: &'a str,
    pub(crate) value: i64,
    pub(crate) ttl: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct IncrementCounterRequest<'a> {
    pub(crate) key: &'a str,
    pub(crate) amount: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct DecrementCounterRequest<'a> {
    pub(crate) key: &'a str,
    pub(crate) amount: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct SetCounterRequest<'a> {
    pub(crate) key: &'a str,
    pub(crate) value: i64,
}
