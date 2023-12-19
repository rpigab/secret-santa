use crate::obfuscate::obfuscate_name;

pub fn build_recipient_uri(base_uri: &str, giver: &str, recipient: &str, seed: usize, max_length: usize) -> String {
    let recipient_obf = obfuscate_name(recipient, seed, max_length);
    format!("{base_uri}?g={giver}&s={seed}&r={recipient_obf}")
}

#[cfg(test)]
mod tests {
    use crate::recipient_uri::build_recipient_uri;

    #[test]
    fn build_recipient_uri_ok() {
        let res = build_recipient_uri("http://localhost:8080/fr/affectation-v2.html",
                                      "Alice", "Bob", 42, 20);
        assert_eq!(res, "http://localhost:8080/fr/affectation-v2.html?g=Alice&s=42&r=aEVICgoKCgoKCgoKCgoKCgoKCgo");
    }
}
