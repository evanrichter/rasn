#[macro_export]
macro_rules! roundtrip {
    ($data:expr, $type:ty) => {{
        use ::rasn::der::decode;
        use ::rasn::der::encode;
        if let Ok(value) = decode::<$type>($data) {
            assert_eq!(value, decode(&encode(&value).unwrap()).unwrap());
        }
    }};
}
