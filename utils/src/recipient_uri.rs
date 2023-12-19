use base64::Engine;
use base64::engine::general_purpose;

use crate::relative_uri::build_relative_uri;

pub fn build_recipient_uri(giver: String, recipient: String) -> String {
    let recipient_b64 = general_purpose::STANDARD.encode(recipient);
    let query_params = vec![
        ("giver".to_string(), giver),
        ("recipient".to_string(), recipient_b64),
    ];
    build_relative_uri(query_params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_recipient_uri_ok() {
        assert_eq!("?giver=Bob&recipient=QWxpY2U%3D".to_string(),
                   build_recipient_uri("Bob".to_string(), "Alice".to_string()))
    }
}