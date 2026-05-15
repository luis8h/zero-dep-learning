//! This simple implementation of a bit reader reads bits one by one. It could be improved by a bulk
//! method, but then it would be better to use word-sized input (u64) instead of u8 to really see
//! the advantage

#![allow(unused)]

use super::Bit;
use super::BitResult;
use std::fmt;
use std::slice::Iter;

pub struct BitReader<'a> {
    position: u8, // bit mask
    buffer: u8,
    content: Iter<'a, u8>,
}

#[derive(Debug, PartialEq)]
pub enum BitReaderError {
    EndOfStream,
}
impl fmt::Display for BitReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitReaderError::EndOfStream => write!(f, "Reached end of bitstream unexpectedly"),
        }
    }
}
impl std::error::Error for BitReaderError {}

impl<'a> BitReader<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        BitReader {
            position: 0,
            content: content.iter(),
            buffer: 0,
        }
    }

    /// Reads the next Bit of a BitStream.
    pub fn next_bit(&mut self) -> Result<Bit, BitReaderError> {
        // reset position after overflow and assign next byte as buffer
        if self.position == 0 {
            self.position = 0b00000001;
            self.buffer = *self.content.next().ok_or(BitReaderError::EndOfStream)?;
        }

        // read current bit
        let bit: Bit = ((self.buffer & self.position) > 0).into();

        // update position for next read
        self.position <<= 1;

        Ok(bit)
    }

    /// Reads the next n bits from the BitStream.
    /// The result will be right Alligned, so when reading 4 bits, the 4 rightmost bits of the u32 value will be used.
    /// When reading over a byte border, the bits of the right byte end up before the bits of the
    /// left byte (eg. [0b00110011, 0b11001100] -> 0b110000110011 when reading 12 bits).
    /// It returns a BitResult containing just as many u32 values as needed.
    /// When interpreting the BitResult, the length of it needs to be used to get a correct interpretation.
    pub fn next_bits(&mut self, n: usize) -> Result<BitResult, BitReaderError> {
        // construct output vector with just as many u32 values as needed to store a result with
        // length n
        let mut out: Vec<u32> = vec![0u32; (n + 31) / 32];

        for i in 0..n {
            if self.next_bit()? == Bit::One {
                // if the value is 1, or it into the output vector at the correct position
                out[i / 32] |= 1 << (i % 32);
            }
        }

        Ok(BitResult::try_from((out, n)).expect("Logical Error in BitReader constructing a wrong BitResult."))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_bit() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100, 1, 110]);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
    }

    #[test]
    fn next_bit_over_byte_border() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100, 1, 110]);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
        assert_eq!(reader.next_bit().unwrap(), Bit::One);
    }

    #[test]
    fn next_bit_end_of_stream_error() {
        let mut reader = BitReader::new(&[0]);

        for i in 0..8 {
            assert_eq!(reader.next_bit().unwrap(), Bit::Zero);
        }

        let result = reader.next_bit();

        assert_eq!(result, Err(BitReaderError::EndOfStream));
    }

    #[test]
    fn next_bits() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100]);
        assert_eq!(
            reader.next_bits(4).unwrap(),
            BitResult::try_from((vec![0b0011], 4)).unwrap()
        );
    }

    #[test]
    fn next_bits_over_byte_border() {
        let mut reader = BitReader::new(&[0b00110011, 0b11001100]);
        assert_eq!(
            reader.next_bits(12).unwrap(),
            BitResult::try_from((vec![0b110000110011], 12)).unwrap()
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

        assert_eq!(result.unwrap(), BitResult::try_from((expected_vec, 34)).unwrap());
    }

    #[test]
    fn next_bits_end_of_stream_error() {
        let mut reader = BitReader::new(&[0, 1]);
        let result = reader.next_bits(17);
        assert_eq!(result, Err(BitReaderError::EndOfStream));
    }
}
