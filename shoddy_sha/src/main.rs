use std::fmt;
use std::fmt::Formatter;
use primal;
use std::convert::TryInto;

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
    padded.extend(vec![0x70_u8]);
    padded.extend(vec![0; required_padding_bytes]);
    padded.extend(message_len.to_be_bytes().iter());

    padded
}

fn main() {
    let k = generate_round_constants();
    let mut h = generate_initial_hash_values();

    println!("k: {:08x?}\nh: {:08x?}", k, h);

    let message = String::from("Hello,world").into_bytes();
    // let message = vec!{0xff_u8; 512/8};
    let padded = pad(&message);
    println!("({}) {:02x?}", padded.len() * 8, padded);

    for chunk in padded.chunks_exact(512/8) {
        print!("Got chunk: {:02x?}\n", chunk);
        // Create schedule
        let mut schedule = [0_u32; 64];
        // Copy the chunk into the schedule
        chunk.chunks_exact(chunk.len() / 16)
            .enumerate()
            .for_each(|(i,w)| schedule[i] = u32::from_be_bytes(w.try_into().expect("chunk chunk wrong size")));
        print!("Init'd schedule: {:02x?}\n", &schedule);
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

    // #[test]
    // fn test_get_bit() {
    //     let msb = 0x80_00_00_00_00_00_00_00_u64;
    //     assert_eq!(get_bit(msb, 63), '1');
    //
    //     let lsb = 0x1_u64;
    //     assert_eq!(get_bit(lsb, 0), '1');
    // }
}
