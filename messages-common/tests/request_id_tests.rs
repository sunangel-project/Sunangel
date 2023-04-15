use bytes::Bytes;
use messages_common::{get_request_id, try_get_request_id};

fn assert_try_get(payload: String, expected: String) {
    let bytes = Bytes::from(payload);
    let result = try_get_request_id(&bytes);

    match result {
        Ok(request_id) => assert_eq!(expected, request_id),
        Err(err) => panic!("Didn't expect error {err:?}"),
    }
}

fn error_try_get(payload: String) {
    let bytes = Bytes::from(payload);
    let result = try_get_request_id(&bytes);

    match result {
        Err(_) => (),
        Ok(_) => panic!("Expected function to return error"),
    }
}

fn assert_get(payload: String, expected: String) {
    let bytes = Bytes::from(payload);
    let request_id = get_request_id(&bytes);

    assert_eq!(expected, request_id)
}

#[test]
fn test_try_get_succesful() {
    assert_try_get("{\"request_id\": \"test-id\"}".into(), "test-id".into());
    assert_try_get(
        "{\"request_id\": \"18f75a98-bdbe-432d-8e92-3c20c6153150\"}".into(),
        "18f75a98-bdbe-432d-8e92-3c20c6153150".into(),
    );

    assert_try_get(
        "{\"random_attribute\": true,\"request_id\": \"test-id\"}".into(),
        "test-id".into(),
    )
}

#[test]
fn test_try_fail() {
    error_try_get("".into());
    error_try_get("{}".into());
    error_try_get("{\"random_attribute\": false}".into());
}

#[test]
fn test_get_succesful() {
    assert_get("{\"request_id\": \"test-id\"}".into(), "test-id".into());
    assert_get(
        "{\"request_id\": \"18f75a98-bdbe-432d-8e92-3c20c6153150\"}".into(),
        "18f75a98-bdbe-432d-8e92-3c20c6153150".into(),
    );

    assert_get(
        "{\"random_attribute\": true,\"request_id\": \"test-id\"}".into(),
        "test-id".into(),
    )
}

#[test]
fn test_get_empty() {
    assert_get("".into(), "".into());
    assert_get("{}".into(), "".into());
    assert_get("{\"random_attribute\": false}".into(), "".into());
}
