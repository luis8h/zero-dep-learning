use std::collections::VecDeque;

use super::Bit;

pub struct BitReader {
    position: u8, // binary mask
    buffer: u8,
    content: VecDeque<u8>, // i think this could also be a slice in theory, but then the reader would
                      // need to keep track of the current index
}
//
// WARN: use iterator for cleaner API design: https://gemini.google.com/app/e457285b11281572?hl=de
// The last approach or even the first one by implementing the BitReader directly as a iterator
// might be even cleaner.
//
// EDIT:
// use a slice as input
// implement a read_n_bits method which returns a BitResult (can contain any number of bits) (https://gemini.google.com/app/53478774b716bb6c?hl=de)
// the bit result has methods for eg. retrieving a number, string etc. from the bits
// there should be also a trait that can be implemented so that the caller can build his own parses
// for bit result
//
// the BitResult does not use Bit but u8 oderso for better performance
//
// Also document why other approaches like the

impl BitReader {
    pub fn new(mut content: VecDeque<u8>) -> Self {
        // TODO: maybe do this in the next bit method?
        let buffer = content.pop_front().expect("Cannot read from empty content");

        BitReader {
            position: 0b00000001,
            buffer,
            content,
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        // read current bit
        let bit: Bit = ((self.buffer & self.position) > 0).into();

        // update position for next read
        self.position <<= 1;

        // reset position after overflow and assign next byte as buffer
        if self.position == 0 {
            self.position = 0b00000001;
            self.buffer = self.content.pop_front().expect("End of content is not handled right now");
        }

        bit
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let mut reader = BitReader::new(vec![7, 5]);
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
        println!("{}", reader.next_bit());
    }
}
