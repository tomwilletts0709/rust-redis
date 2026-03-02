use rust_redis::Storage;

#[test]
fn test_ping() {
    let mut storage = Storage::new();
    let result = storage.process_command(&vec!["PING".to_string()]);
    assert!(result.is_ok());
}

#[test]
fn test_set_get() {
    let mut storage = Storage::new();
    let set_result = storage.process_command(&vec!["SET".to_string(), "foo".to_string(), "bar".to_string()]);
    assert!(set_result.is_ok());

    let get_result = storage.process_command(&vec!["GET".to_string(), "foo".to_string()]);
    assert!(get_result.is_ok());
    if let Ok(rust_redis::RESP::BulkString(s)) = get_result {
        assert_eq!(s, "bar");
    }
}