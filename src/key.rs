use super::constants::BYTES_PER_KEY;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Key([u64; BYTES_PER_KEY]);

impl Key {
    pub fn new(input: String) -> Key {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();

        let mut result = [0; BYTES_PER_KEY];

        for i in (0..hash.len()).step_by(8) {
            let index = i / 8;
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&hash[i..(i + 8)]);
            result[index] = u64::from_le_bytes(bytes);
        }

        Key(result)
    }

    pub fn distance(&self, y: &Key) -> Key {
        let mut result = [0; BYTES_PER_KEY];

        for i in 0..BYTES_PER_KEY {
            result[i] = self.0[i] ^ y.0[i];
        }

        Key(result)
    }
}
