// Implementation of SHA256 based on the pseudocode implementation
// outlined at Wikipedia: https://en.wikipedia.org/wiki/SHA-2#Pseudocode
use std::convert::TryInto;

pub fn one_shot(message: &[u8]) -> Vec<u8> {
    let k = crate::generate_round_constants();
    let mut hash = crate::generate_initial_hash_values();

    for chunk in pad(&message).chunks_exact(512/8) {
        // Create schedule
        let mut schedule = [0_u32; 64];

        // Copy the chunk into the schedule
        chunk.chunks_exact(chunk.len() / 16)
            .enumerate()
            .for_each(|(i,w)| schedule[i] = u32::from_be_bytes(w.try_into().expect("chunk chunk wrong size")));

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
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(schedule[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

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
    }

    let mut digest: Vec<u8> = Vec::with_capacity(256/8);
    for part in hash.iter() {
        digest.extend(part.to_be_bytes().iter());
    }

    digest
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

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Sha256, Digest};
    use rand::RngCore;

    // TODO: Move this to the super module
    const CHUNK_BYTE_SIZE: usize = 512 / 8;

    /// The max message is calculated as:
    ///      CHUNK_BYTE_SIZE - size+of_terminator_byte - size_of_message_length_value
    const MAX_SINGLE_CHUNK_MESSAGE_BYTE_SIZE: usize = CHUNK_BYTE_SIZE - 1 - (64 / 8);

    #[test]
    fn test_padding_empty_message() {
        let subject = pad(&[]);
        assert_eq!(subject.len(), 512/8);
    }

    #[test]
    fn test_maximum_single_chunk() {
        let max_single_chunk_message = [0xff; MAX_SINGLE_CHUNK_MESSAGE_BYTE_SIZE];
        let subject = pad(&max_single_chunk_message);
        assert_eq!(subject.len(), 512/8);

        let one_over_max_single_chunk_message = [0xff; MAX_SINGLE_CHUNK_MESSAGE_BYTE_SIZE + 1];
        let subject = pad(&one_over_max_single_chunk_message);
        assert_eq!(subject.len(), 512/8 * 2);
    }

    #[test]
    fn test_simple_padded_message() {
        let subject = pad(&String::from("hello world").into_bytes());
        let expected = vec![
            0b01101000u8, 0b01100101u8, 0b01101100u8, 0b01101100u8, 0b01101111u8, 0b00100000u8, 0b01110111u8, 0b01101111u8,
            0b01110010u8, 0b01101100u8, 0b01100100u8, 0b10000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8,
            0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b01011000u8,
        ];
        assert_eq!(subject, expected);
    }

    #[test]
    fn test_output() {
        let message = String::from("hello world").into_bytes();
        let subject = one_shot(&message);
        let expected = Sha256::digest(&message);
        assert_eq!(&expected.as_slice(), &subject);

        let mut message = [0u8; 1025];
        rand::thread_rng().fill_bytes(&mut message);
        let subject = one_shot(&message);
        let expected = Sha256::digest(&message);
        assert_eq!(&expected.as_slice(), &subject);
    }
}