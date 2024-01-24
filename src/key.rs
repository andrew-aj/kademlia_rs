use super::constants::BYTES_PER_KEY;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Key([u8; BYTES_PER_KEY]);

impl Key {
    pub fn new(input: String) -> Key {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();

        Key(hash.into())
    }

    pub fn distance(&self, y: &Key) -> Key {
        let mut result = [0; BYTES_PER_KEY];

        for i in 0..BYTES_PER_KEY {
            result[i] = self.0[i] ^ y.0[i];
        }

        Key(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_endian() {
        let input = "hello world".to_owned();
        let key = Key::new(input);

        let expected = [
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
            0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
            0xe2, 0xef, 0xcd, 0xe9,
        ];

        assert_eq!(key.0, expected);
    }
}
