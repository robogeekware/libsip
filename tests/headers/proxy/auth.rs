use libsip::{headers::parse::parse_proxy_authenticate_header, AuthHeader, AuthSchema, Header};
use nom::error::VerboseError;
use std::collections::HashMap;

#[test]
fn write() {
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::ProxyAuthenticate(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        "Proxy-Authenticate: Digest key=\"value\"".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::ProxyAuthenticate(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_proxy_authenticate_header::<VerboseError<&[u8]>>(
            b"Proxy-Authenticate: Digest key=value \r\n"
        )
    );
}
