//! Utilities for working with messages used internally by the sunangel-project

use anyhow::anyhow;
use bytes::Bytes;
use log::error;
use serde_json::Value;
use std::{
    error::Error,
    str::{self, FromStr},
};

const REQUEST_ID_KEY: &str = "request_id";

/// Try to decode the request id of a message
///
/// If successful it will return `Ok(request_id)`,
/// otherwise it will return an error `Err(error)`.
///
/// # Examples
///
/// ```
/// use bytes::Bytes;
/// use messages_common::try_get_request_id;
///
/// let payload = Bytes::from("{\"request_id\": \"18f75a98-bdbe-432d-8e92-3c20c6153150\"}");
/// let res_request_id = try_get_request_id(&payload);
///
/// if let Ok(request_id) = res_request_id {
///     assert_eq!(request_id, "18f75a98-bdbe-432d-8e92-3c20c6153150".to_string())
/// }
/// ```
///
/// ## Error
///
/// ```
/// use bytes::Bytes;
/// use messages_common::try_get_request_id;
///
/// let payload = Bytes::from("{}");
/// let res_request_id = try_get_request_id(&payload);
///
/// if let Err(error) = res_request_id {
///     assert_eq!(error.to_string(), "expected object to have key request_id".to_string())
/// }
/// ```
///
/// This is just one possible error message.
pub fn try_get_request_id(payload: &Bytes) -> Result<String, Box<dyn Error + Send + Sync>> {
    let payload_str = str::from_utf8(payload)?;
    let payload_val = Value::from_str(payload_str)?;

    let payload_obj = payload_val
        .as_object()
        .ok_or(anyhow!("expected payload to be object"))?;

    let request_id_val = payload_obj
        .get(&REQUEST_ID_KEY.to_string())
        .ok_or(anyhow!("expected object to have key {REQUEST_ID_KEY}"))?;

    let request_id = request_id_val
        .as_str()
        .ok_or(anyhow!(
            "request_id was {} not a string",
            request_id_val.to_string()
        ))?
        .to_string();

    Ok(request_id)
}

/// Decode the request id of a message
///
/// If successful it will return the request id,
/// otherwise it will return an empty string as the request id.
///
/// # Examples
///
/// ```
/// use bytes::Bytes;
/// use messages_common::get_request_id;
///
/// let payload = Bytes::from("{\"request_id\": \"18f75a98-bdbe-432d-8e92-3c20c6153150\"}");
/// let request_id = get_request_id(&payload);
///
/// assert_eq!(request_id, "18f75a98-bdbe-432d-8e92-3c20c6153150".to_string())
/// ```
///
/// ## Fail
///
/// ```
/// use bytes::Bytes;
/// use messages_common::get_request_id;
///
/// let payload = Bytes::from("{}");
/// let request_id = get_request_id(&payload);
///
/// assert_eq!(request_id, "".to_string())
/// ```
pub fn get_request_id(payload: &Bytes) -> String {
    let res_request_id = try_get_request_id(payload);

    res_request_id.unwrap_or_else(|err| {
        error!("Couldn't get request_id from {payload:?}, because '{err}'");

        "".to_string()
    })
}
