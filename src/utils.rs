use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn hash_string(string: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    format!("{:X}", hasher.finalize())
}

pub fn generate_uuid() -> String {
    String::from(
        Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    )
}