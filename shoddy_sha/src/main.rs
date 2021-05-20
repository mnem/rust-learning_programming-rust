use std::fmt;
use std::fmt::Formatter;
use primal;
use std::convert::TryInto;
use sha2::{Sha256, Digest};

struct Bytefield<T>(T);

impl Bytefield<f64> {
    fn decompose(&self) -> (u64, u64, u64) {
        let bits = self.0.to_bits();
        let sign = (bits >> 63) & 0x1_u64;
        let exponent = (bits >> 52) & 0x07ff_u64;
        let mantissa = bits & 0xf_ffff_ffff_ffff_u64;

        (sign, exponent, mantissa)
    }
}

impl fmt::LowerHex for Bytefield<f64> {
    fn fmt(&self,f: &mut Formatter<'_>) -> fmt::Result {
        let (sign, exponent, mantissa) = self.decompose();
        f.write_fmt(format_args!("{:x} {:03x} {:013x}", sign, exponent, mantissa))
    }
}

impl fmt::Binary for Bytefield<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (sign, exponent, mantissa) = self.decompose();
        f.write_fmt(format_args!("{:b} {:011b} {:052b}", sign, exponent, mantissa))
    }
}

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

fn generate_round_constants() -> [u32; 64] {
    let mut constants = [0; 64];
    for (i, n) in primal::Primes::all().take(64).enumerate() {
        constants[i] = ((n as f64).cbrt().fract() * 16_f64.powf(8_f64)) as u32;
    }
    constants
}

fn generate_initial_hash_values() -> [u32; 8] {
    let mut constants = [0; 8];
    for (i, n) in primal::Primes::all().take(8).enumerate() {
        constants[i] = ((n as f64).sqrt().fract() * 16_f64.powf(8_f64)) as u32;
    }
    constants
}

fn pad(m: &[u8]) -> Vec<u8> {
    let message_len = m.len();
    let required_padding_bits = 512 - ((message_len * 8 + 1 + 64) % 512);
    let required_padding_bytes = ((required_padding_bits as f64) / 8_f64).ceil() as usize;
    let required_padding_bytes = required_padding_bytes - 1; // We always pad with 0x70_u8

    let mut padded = Vec::with_capacity(m.len() + 1 + required_padding_bytes + (64/8));
    padded.extend(m);
    padded.extend(vec![0x80_u8]);
    padded.extend(vec![0; required_padding_bytes]);
    padded.extend((message_len * 8).to_be_bytes().iter());

    padded
}

fn main() {
    let k = generate_round_constants();
    let mut hash = generate_initial_hash_values();

    println!("k: {:08x?}\nhash: {:08x?}", k, hash);

    let message = String::from("hello world").into_bytes();
    let padded = pad(&message);
    println!("({}) {:02x?}", padded.len() * 8, padded);

    let expected_padded = vec![
        0b01101000u8, 0b01100101u8, 0b01101100u8, 0b01101100u8, 0b01101111u8, 0b00100000u8, 0b01110111u8, 0b01101111u8,
        0b01110010u8, 0b01101100u8, 0b01100100u8, 0b10000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b01011000u8,
    ];
    assert_eq!(padded, expected_padded, "Padding is unexpected");

    for chunk in padded.chunks_exact(512/8) {
        print!("Got chunk: {:02x?}\n", chunk);
        // Create schedule
        let mut schedule = [0_u32; 64];
        // Copy the chunk into the schedule
        chunk.chunks_exact(chunk.len() / 16)
            .enumerate()
            .for_each(|(i,w)| schedule[i] = u32::from_be_bytes(w.try_into().expect("chunk chunk wrong size")));
        print!("Init'd schedule: {:02x?}\n", &schedule);

        // Extend
        for i in 16..64 {
            let s0 = schedule[i - 15].rotate_right(7) ^ schedule[i- 15].rotate_right(18) ^ (schedule[i - 15] >> 3);
            let s1 = schedule[i - 2].rotate_right(17) ^ schedule[i - 2].rotate_right(19) ^ (schedule[i - 2] >> 10);
            schedule[i] = schedule[i - 16].wrapping_add(s0).wrapping_add(schedule[i - 7]).wrapping_add(s1);
        }

        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];
        let mut e = hash[4];
        let mut f = hash[5];
        let mut g = hash[6];
        let mut h = hash[7];

        for i in 0..64 {
            let S1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(S1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(schedule[i]);
            let S0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = S0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        hash[0] = hash[0].wrapping_add(a);
        hash[1] = hash[1].wrapping_add(b);
        hash[2] = hash[2].wrapping_add(c);
        hash[3] = hash[3].wrapping_add(d);
        hash[4] = hash[4].wrapping_add(e);
        hash[5] = hash[5].wrapping_add(f);
        hash[6] = hash[6].wrapping_add(g);
        hash[7] = hash[7].wrapping_add(h);

        let mut digest: Vec<u8> = Vec::with_capacity(256/8);
        for part in hash.iter() {
            digest.extend(part.to_be_bytes().iter());
        }

        let hash_string = digest.iter().map(|n| format!("{:02x}", n)).collect::<String>();
        println!("Hash    : {}", hash_string);
        let expected = Sha256::digest(&message);
        println!("Expected: {:02x}", expected);

    }

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
        let hash = generate_initial_hash_values();
        let expected = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_round_constants() {
        let round_constants = generate_round_constants();
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
