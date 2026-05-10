//! This simple implementation of a bit reader reads bits one by one. It could be improved by a bulk
//! method, but then it would be better to use word-sized input (u64) instead of u8 to really see
//! the advantage

#![allow(unused)]

use super::Bit;
use super::BitResult;
use std::slice::Iter;

pub struct BitReader<'a> {
    position: u8, // bit mask
    buffer: u8,
    content: Iter<'a, u8>,
}

impl<'a> BitReader<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        BitReader {
            position: 0,
            content: content.iter(),
            buffer: 0,
        }
    }

    /// Reads the next Bit of a BitStream.
    pub fn next_bit(&mut self) -> Bit {
        // reset position after overflow and assign next byte as buffer
        if self.position == 0 {
            self.position = 0b00000001;
            self.buffer = *self
                .content
                .next()
                .expect("Empty buffer not handlred currently");
        }

        // read current bit
        let bit: Bit = ((self.buffer & self.position) > 0).into();

        // update position for next read
        self.position <<= 1;

        bit
    }

    /// Reads the next n bits from the BitStream.
    /// The result will be right Alligned, so when reading 4 bits, the 4 rightmost bits of the u32 value will be used.
    /// When reading over a byte border, the bits of the right byte end up before the bits of the
    /// left byte (eg. [0b00110011, 0b11001100] -> 0b110000110011 when reading 12 bits).
    /// It returns a BitResult containing just as many u32 values as needed.
    /// When interpreting the BitResult, the length of it needs to be used to get a correct interpretation.
    pub fn next_bits(&mut self, n: usize) -> BitResult {
        // construct output vector with just as many u32 values as needed to store a result with
        // length n
        let mut out: Vec<u32> = vec![0u32; (n + 31) / 32];

        for i in 0..n {
            if self.next_bit() == Bit::One {
                // if the value is 1, or it into the output vector at the correct position
                out[i / 32] |= 1 << (i % 32);
            }
        }

        BitResult::new(out, n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_bit() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100, 1, 110]);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::Zero);
    }

    #[test]
    fn next_bit_over_byte_border() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100, 1, 110]);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::Zero);
        assert_eq!(reader.next_bit(), Bit::One);
        assert_eq!(reader.next_bit(), Bit::One);
    }

    #[test]
    fn next_bits() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100]);
        assert_eq!(dbg!(reader.next_bits(4)), BitResult::new(vec![0b0011], 4));
    }

    #[test]
    fn next_bits_over_byte_border() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100]);
        assert_eq!(
            reader.next_bits(12),
            BitResult::new(vec![0b110000110011], 12)
        );
    }

    #[test]
    fn next_bits_more_then_32() {
        let data = [0xAA, 0x00, 0x00, 0x00, 0x03];
        let mut reader = BitReader::new(&data);

        let result = reader.next_bits(34);

        let expected_vec = vec![
            0x000000AA, // The first 32 bits
            0x00000003, // The next 2 bits (from the 5th byte)
        ];

        assert_eq!(result, BitResult::new(expected_vec, 34));
    }
}
