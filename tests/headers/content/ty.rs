use libsip::headers::{parse::parse_content_type_header, ContentType, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ContentType(ContentType::Sdp);
    assert_eq!(
        "Content-Type: application/sdp".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ContentType(ContentType::Sdp);
    let new_header= parse_content_type_header::<VerboseError<&[u8]>>(b"Content-Type: application/sdp\r\n");
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_content_type_header::<VerboseError<&[u8]>>(b"Content-Type: application/sdp\r\n")
    );
    let m = match new_header.unwrap().1 {
         Header::ContentType(ContentType::Sdp) => "application/sdp",
         _ => ""
    };
    assert_eq!(m, "application/sdp");

}
