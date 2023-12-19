use base64::Engine;
use base64::engine::general_purpose;
use general_purpose::URL_SAFE_NO_PAD;
use rand::Rng;

/// Pad strings to the length of the longest one with spaces,
/// then generate a random seed,
/// use it to perform a basic two-way encryption on it,
/// encode to base64 each string, and url encode each, then return the array and the seed
pub fn obfuscate_names(names: Vec<&str>, seed: usize) -> Vec<String> {
    let max_length = names.iter().map(|s| s.len()).max().unwrap_or(0);

    let padded_names: Vec<String> = names
        .iter()
        .map(|name| name.chars().chain(std::iter::repeat(' ').take(max_length - name.len())).collect())
        .collect();

    let obfuscated_names: Vec<String> = padded_names
        .iter()
        .map(|name| {
            let encrypted = two_way_cipher(name, seed);
            URL_SAFE_NO_PAD.encode(encrypted)
        })
        .collect();

    obfuscated_names
}

pub fn obfuscate_name(name: &str, seed: usize, max_length: usize) -> String {
    let padded_name: String = name.chars()
        .chain(std::iter::repeat(' ')
            .take(max_length - name.len()))
        .collect();
    let encrypted = two_way_cipher(&padded_name, seed);
    URL_SAFE_NO_PAD.encode(encrypted)
}

/// The reverse of obfuscate_names() on a single name, given the seed
pub fn deobfuscate_name(encoded: &str, seed: &str) -> String {
    let seed: usize = seed.parse().unwrap();
    let base64_decoded = URL_SAFE_NO_PAD.decode(encoded).unwrap();
    let base64_decoded = String::from_utf8(base64_decoded).unwrap();
    let decrypted = two_way_cipher(&base64_decoded, seed);
    decrypted.trim().to_string()
}

pub fn generate_random_seed() -> usize {
    rand::thread_rng().gen_range(1..usize::MAX)
}

fn two_way_cipher(input: &str, seed: usize) -> String {
    input.chars()
        .map(|c| (c as usize ^ seed) as u8 as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obf() {
        let names = vec![
            "Alice",
            "Bob",
            "Carol",
            "David",
            "Z",
            "Supercalifragilisticexpialidocious",
            "",
        ];
        let seed = 15;

        let obfuscated_names = obfuscate_names(names, seed);

        let res = vec![
            "TmNmbGovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "TWBtLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "TG59YGMvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "S255ZmsvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "VS8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "XHp_an1sbmNmaX1uaGZjZnx7Zmxqd39mbmNma2BsZmB6fA",
            "Ly8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
        ];
        assert_eq!(res, obfuscated_names);
    }

    #[test]
    fn deobf() {
        let obf = vec![
            "TmNmbGovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "TWBtLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "TG59YGMvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "S255ZmsvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "VS8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
            "XHp_an1sbmNmaX1uaGZjZnx7Zmxqd39mbmNma2BsZmB6fA",
            "Ly8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLw",
        ];
        let seed = "15";

        let names = vec![
            "Alice",
            "Bob",
            "Carol",
            "David",
            "Z",
            "Supercalifragilisticexpialidocious",
            "",
        ];

        for (i, &obf_s) in obf.iter().enumerate() {
            let deobf = deobfuscate_name(obf_s, seed);
            println!("deobf:-{deobf}-");
            assert_eq!(names[i], deobf);
        }
    }
}
