use mailkit::JsonValue;

#[test]
fn macro_object() {
    let val = mailkit::json!({"a": 1u64, "b": true});
    if let JsonValue::Object(map) = val {
        assert_eq!(map.get("a"), Some(&JsonValue::Number(1.0)));
        assert_eq!(map.get("b"), Some(&JsonValue::Bool(true)));
    } else {
        panic!("not object");
    }
}

#[test]
fn macro_array() {
    let val = mailkit::json!([1u64, 2u64, 3u64]);
    assert_eq!(val, JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0), JsonValue::Number(3.0)]));
}

#[test]
fn macro_object_ident() {
    let val = mailkit::json!({a: 1u64, b: true});
    if let JsonValue::Object(map) = val {
        assert_eq!(map.get("a"), Some(&JsonValue::Number(1.0)));
        assert_eq!(map.get("b"), Some(&JsonValue::Bool(true)));
    } else {
        panic!("not object");
    }
}
