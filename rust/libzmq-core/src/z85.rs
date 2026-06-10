// SPDX-License-Identifier: MPL-2.0

const ENCODER: &[u8; 85] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";

const DECODER: [u8; 96] = [
    0xFF, 0x44, 0xFF, 0x54, 0x53, 0x52, 0x48, 0xFF, 0x4B, 0x4C, 0x46, 0x41, 0xFF, 0x3F, 0x3E, 0x45,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x40, 0xFF, 0x49, 0x42, 0x4A, 0x47,
    0x51, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32,
    0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x4D, 0xFF, 0x4E, 0x43, 0xFF,
    0xFF, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x4F, 0xFF, 0x50, 0xFF, 0xFF,
];

pub fn encoded_len(size: usize) -> Option<usize> {
    if size % 4 == 0 {
        Some(size / 4 * 5)
    } else {
        None
    }
}

pub fn decoded_len(encoded: &[u8]) -> Option<usize> {
    if encoded.len() >= 5 && encoded.len() % 5 == 0 {
        Some(encoded.len() / 5 * 4)
    } else {
        None
    }
}

pub fn encode(data: &[u8], dest: &mut [u8]) -> bool {
    let Some(output_len) = encoded_len(data.len()) else {
        return false;
    };
    if dest.len() < output_len + 1 {
        return false;
    }

    let mut char_nbr = 0;
    for chunk in data.chunks_exact(4) {
        let mut value = 0u32;
        for byte in chunk {
            value = value * 256 + u32::from(*byte);
        }

        let mut divisor = 85u32.pow(4);
        while divisor != 0 {
            dest[char_nbr] = ENCODER[(value / divisor % 85) as usize];
            char_nbr += 1;
            divisor /= 85;
        }
    }

    dest[char_nbr] = 0;
    true
}

pub fn decode(encoded: &[u8], dest: &mut [u8]) -> bool {
    let Some(output_len) = decoded_len(encoded) else {
        return false;
    };
    if dest.len() < output_len {
        return false;
    }

    let mut byte_nbr = 0;
    let mut value = 0u32;
    for (char_nbr, byte) in encoded.iter().enumerate() {
        if u32::MAX / 85 < value {
            return false;
        }
        value *= 85;

        let index = byte.wrapping_sub(32);
        if usize::from(index) >= DECODER.len() {
            return false;
        }

        let summand = u32::from(DECODER[usize::from(index)]);
        if summand == 0xFF || summand > u32::MAX - value {
            return false;
        }
        value += summand;

        if (char_nbr + 1) % 5 == 0 {
            let mut divisor = 256u32.pow(3);
            while divisor != 0 {
                dest[byte_nbr] = (value / divisor % 256) as u8;
                byte_nbr += 1;
                divisor /= 256;
            }
            value = 0;
        }
    }

    byte_nbr == output_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_rfc32_test_vector() {
        let data = [0x86, 0x4F, 0xD2, 0x6F, 0xB5, 0x59, 0xF7, 0x5B];
        let mut out = [0u8; 11];
        assert!(encode(&data, &mut out));
        assert_eq!(&out, b"HelloWorld\0");
    }

    #[test]
    fn decodes_rfc32_test_vector() {
        let mut out = [0u8; 8];
        assert!(decode(b"HelloWorld", &mut out));
        assert_eq!(out, [0x86, 0x4F, 0xD2, 0x6F, 0xB5, 0x59, 0xF7, 0x5B]);
    }

    #[test]
    fn rejects_invalid_lengths_and_values() {
        let mut encoded = [0u8; 64];
        assert!(!encode(&[1, 2, 3], &mut encoded));

        let mut decoded = [0u8; 8];
        assert!(!decode(b"0", &mut decoded));
        assert!(!decode(b"01234567", &mut decoded));
        assert!(!decode(b"#####", &mut decoded));
        assert!(!decode(b"%nSc1", &mut decoded));
        assert!(!decode(b"####\x7f", &mut decoded));
        assert!(!decode(b"####\x1f", &mut decoded));
    }

    #[test]
    fn roundtrips_min_and_max_words() {
        for data in [[0u8; 4], [0xffu8; 4]] {
            let mut encoded = [0u8; 6];
            let mut decoded = [0u8; 4];
            assert!(encode(&data, &mut encoded));
            assert!(decode(&encoded[..5], &mut decoded));
            assert_eq!(decoded, data);
        }
    }
}
