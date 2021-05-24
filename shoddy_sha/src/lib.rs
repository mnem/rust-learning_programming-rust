pub mod wp_sha256;
pub mod cms_sha256;
pub mod real_sha256;
pub mod inspect;

use primal;

/// Hasher trait for simpler comparisons
pub trait Hasher {
    fn one_shot(message: &[u8]) -> Vec<u8>;
}

/// Generates the round constants for SHA 256
pub fn generate_round_constants() -> [u32; 64] {
    let mut constants = [0; 64];
    for (i, n) in primal::Primes::all().take(64).enumerate() {
        constants[i] = ((n as f64).cbrt().fract() * 16_f64.powf(8_f64)) as u32;
    }
    constants
}

/// Generates the initial hash values for SHA 256
pub fn generate_initial_hash_values() -> [u32; 8] {
    let mut constants = [0; 8];
    for (i, n) in primal::Primes::all().take(8).enumerate() {
        constants[i] = ((n as f64).sqrt().fract() * 16_f64.powf(8_f64)) as u32;
    }
    constants
}
