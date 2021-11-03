use config_compiler::config::HeaderStruct;
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};

/// Masks static or incoming request header values to the
/// target header list.
///
/// ** Args:
/// 1st arg: Header values to be masked
/// 2nd arg: Header values of the current request that may be
/// required for the header linking
/// 3rd arg: Target HeaderMap where the headers will be masked
#[inline]
pub fn header_masker(
    mask_list: &Option<Vec<HeaderStruct>>,
    incoming_headers: &HeaderMap<HeaderValue>,
    outgoing_headers: &mut HeaderMap<HeaderValue>,
) {
    mask_list.iter().flatten().for_each(|header| {
        // add or drop the header depending on the header.value existing
        match &header.value {
            Some(header_value) => {
                match header_value.chars().next().unwrap() {
                    // Header linking process
                    '$' => {
                        let slice = &header_value[1..];
                        let linked_header = incoming_headers.get(slice);

                        // Check if linked header exists
                        if let Some(linked_header) = linked_header {
                            let value = linked_header.clone();
                            outgoing_headers.insert(
                                HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                                value,
                            );
                        }
                    }
                    _ => {
                        outgoing_headers.insert(
                            HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                            header_value.parse().unwrap(),
                        );
                    }
                }
            }
            None => {
                outgoing_headers.remove(&header.key);
            }
        }
    });
}

#[test]
fn test_header_masker() {
    // Mock headers
    let mut mask_list = Vec::new();
    mask_list.push(HeaderStruct {
        key: "linked-accept-encoding".to_string(),
        value: Some("$accept-encoding".to_string()),
    });

    mask_list.push(HeaderStruct {
        key: "test-header".to_string(),
        value: None,
    });

    mask_list.push(HeaderStruct {
        key: "test-header2".to_string(),
        value: Some("test-value2".to_string()),
    });

    let mut incoming_headers = HeaderMap::new();
    incoming_headers.insert("test-header", "test-value".parse().unwrap());
    incoming_headers.insert("accept-encoding", "gzip".parse().unwrap());

    let mut outgoing_headers = HeaderMap::new();

    // Should apply masking on outgoing_headers
    header_masker(&Some(mask_list), &incoming_headers, &mut outgoing_headers);
    // Shouldn't do anything
    header_masker(&None, &incoming_headers, &mut outgoing_headers);

    assert_eq!(
        outgoing_headers.get("linked-accept-encoding"),
        Some(&HeaderValue::from_static("gzip"))
    );
    assert_eq!(outgoing_headers.get("test-header"), None);
    assert_eq!(
        outgoing_headers.get("test-header2"),
        Some(&HeaderValue::from_static("test-value2"))
    );
}
