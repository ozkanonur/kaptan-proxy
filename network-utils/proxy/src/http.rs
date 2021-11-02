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
