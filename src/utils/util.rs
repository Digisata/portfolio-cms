use hex;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub fn generate_api_key() -> String {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = OsRng; // instantiate the RNG
    let _ = rng.try_fill_bytes(&mut key);
    hex::encode(key)
}
