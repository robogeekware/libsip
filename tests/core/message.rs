use libsip::*;

use nom::error::VerboseError;
use libsip::Header::ContentLength;

#[test]
fn read_message() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .build()
        .unwrap();
    assert_eq!(
        Ok((remains.as_ref(), req)),
        parse_message::<VerboseError<&[u8]>>(b"REGISTER sip:example.com SIP/2.0\r\n\r\n")
    );
}

#[test]
fn parse_content_length() {
    let raw = concat!(
            "INVITE sip:14074560998@67.231.1.154:5060 SIP/2.0\r\n",
            "Via: SIP/2.0/UDP 64.95.59.218:5092;branch=z9hG4bKESg0gXKq4g9Ftph3C35EF7\r\n",
            "Secret-Letter: SuperSecretHeader\r\n",
            "Content-Type: application/sdp\r\n",
            "Content-Length: 128\r\n\r\n").as_bytes();
    let msg = parse_message::<VerboseError<&[u8]>>(raw);
    assert!(msg.is_ok());
    assert!(msg.clone().unwrap().1.headers().content_length().is_some());
    let cl = match msg.unwrap().1.headers().content_length() {
        Some(ContentLength(a)) => a,
        _ => 0,
    };
    assert_eq!(cl, 128);
}

#[test]
fn read_complex() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com"))
        .auth(uri_auth!("user"))
        .parameter(UriParam::RPort(None))
        .parameter(UriParam::Other("new".into(), None))
        .parameter(UriParam::Other("Some".into(), Some("Param".into())))
        .parameter(UriParam::Other("Other".into(), None));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .headers(vec![Header::Expires(10), Header::ContentLength(5)])
        .body(vec![b'6'; 5])
        .build()
        .unwrap();
    assert_eq!(Ok((
        remains.as_ref(), req)),
        parse_message::<VerboseError<&[u8]>>(b"REGISTER sip:user@example.com;rport;new;Some=Param;Other SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n66666")
    );
}
