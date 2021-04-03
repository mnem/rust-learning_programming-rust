fn get_bit(bits: u64, position: usize) -> char {
    assert!(position < 64);
    let mask = 1_u64 << position;
    match bits & mask == mask {
        true => '1',
        false => '0',
    }
}

fn describe(n:f64) -> String {
    let bits = n.to_bits();
    let separator = ' ';

    // Sign bit
    let mut description = String::with_capacity(66);
    description.push(get_bit(bits, 63));
    description.push(separator);

    // Exponent
    for bit in (52..=62).rev() {
        description.push(get_bit(bits, bit));
    }
    description.push(separator);

    // Mantissa
    for bit in (0..=51).rev() {
        description.push(get_bit(bits, bit));
    }

    description
}

fn main() {
    println!("{}", describe(85.125));
    println!("{}", describe(-85.125));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let msb = 0x80_00_00_00__00_00_00_00_u64;
        assert!(get_bit(msb, 63));

        let lsb = 0x1_u64;
        assert!(get_bit(lsb, 0));
    }
}
