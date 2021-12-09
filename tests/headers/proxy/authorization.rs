use libsip::headers::AuthHeader;
use libsip::{headers::parse::parse_proxy_authorization_header, AuthSchema, Header};

use nom::error::VerboseError;
use std::collections::HashMap;

#[test]
fn write() {
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::ProxyAuthorization(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        "Proxy-Authorization: Digest key=\"value\"".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::ProxyAuthorization(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_proxy_authorization_header::<VerboseError<&[u8]>>(
            b"Proxy-Authorization: Digest key=value \r\n"
        )
    );
}
