use sha2::{Sha256, Digest};
use shoddy_sha::inspect::Bytefield;

macro_rules! print_all_hex {
    ($f:expr) => {
        let f = $f;

        println!("{:x}", Bytefield(f));
        println!("{:x}", Bytefield(f * 16_f64.powf(1_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(2_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(3_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(4_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(5_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(6_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(7_f64)));
        println!("{:x}", Bytefield(f * 16_f64.powf(8_f64)));
    };
}

fn hex(n: f64) {
    println!("\n{}", n.cbrt());
    let n = n.cbrt().fract();
    let n = n * 16_f64.powf(8_f64);
    println!("{:x}", Bytefield(n));
    println!("{:b}", Bytefield(n));
    println!("{}", n);
    println!("{}", n as u32);
    println!("{:x}", n as u32);


    // println!("{:x}", Bytefield(n));
    //
    // println!("{:x}", Bytefield(n * 16_f64.powf(1_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(2_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(3_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(4_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(5_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(6_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(7_f64)));
    // println!("{:x}", Bytefield(n * 16_f64.powf(8_f64)));

    // println!("{:x}", Bytefield(n * 16_f64.powf(8_f64)));
    // println!("{:x}", Bytefield(n));
}

fn bin(n: f64) {
    let n = n.cbrt();
    // println!("{:b}", Bytefield(n));
    //
    // println!("{:b}", Bytefield(n * 16_f64.powf(1_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(2_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(3_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(4_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(5_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(6_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(7_f64)));
    // println!("{:b}", Bytefield(n * 16_f64.powf(8_f64)));

    println!("{:b}", Bytefield(n * 16_f64.powf(8_f64)));
}

fn doit(n: f64) {
    hex(n);
    bin(n);
}



fn main() {
    let input = String::from("hello world").into_bytes();
    let digest = shoddy_sha::wp_sha256::one_shot(&input);


    let hash_string = digest.iter().map(|n| format!("{:02x}", n)).collect::<String>();
    println!("Hash    : {}", hash_string);
    let expected = Sha256::digest(&input);
    println!("Expected: {:02x}", expected);

}

fn main2() {
    // doit(2_f64);
    // doit(3_f64);
    // doit(5_f64);

    // let ps :Vec<f64> = primal::Primes::all().take(64).map(|n| n as f64).collect();
    // println!("{:?}",ps);

    print_all_hex!(2_f64);

    // primal::Primes::all()
    //     .take(64)
    //     .map(|n| n as f64)
    //     .for_each(hex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_hash_values() {
        let hash = shoddy_sha::generate_initial_hash_values();
        let expected = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_round_constants() {
        let round_constants = shoddy_sha::generate_round_constants();
        let expected = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
        ];

        assert_eq!(round_constants, expected);
    }
}
