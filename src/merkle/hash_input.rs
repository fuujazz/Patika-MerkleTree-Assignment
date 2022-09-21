use sha2::{Digest, Sha256};

pub fn hash_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(a);
    let result = hasher.finalize();
    let mut hex = String::new();
    for byte in result {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}
