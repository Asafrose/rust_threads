use magic_crypt::{MagicCrypt256, MagicCryptError, MagicCryptTrait, new_magic_crypt};

pub fn encrypt(data: &[u8], key: &str) -> Vec<u8> {
    get_crypt(key).encrypt_to_bytes(data)
}

pub fn decrypt(data: &[u8], key: &str) -> Result<Vec<u8>,MagicCryptError> {
    get_crypt(key).decrypt_bytes_to_bytes(data)
}

fn get_crypt(key: &str) -> MagicCrypt256 {
    new_magic_crypt!(key, 256)
}