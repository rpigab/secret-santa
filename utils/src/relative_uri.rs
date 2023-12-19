use urlencoding::encode;

pub fn build_relative_uri(query_params: Vec<(String, String)>) -> String {
    let params = query_params.into_iter()
        .map(|(k, v)| format!("{}={}", encode(&*k), encode(&*v)))
        .collect::<Vec<String>>()
        .join("&");

    format!("?{params}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_relative_uri_ok() {
        assert_eq!("?giver=Alice&recipient=QWxpY2U%3D",
                   build_relative_uri(vec![
                       ("giver".to_string(), "Alice".to_string()),
                       ("recipient".to_string(), "QWxpY2U=".to_string()),
                   ]))
    }
}