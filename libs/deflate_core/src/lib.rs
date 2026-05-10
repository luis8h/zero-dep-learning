mod bits;

// use std::fs;
use bits::Bit;
use bits::BitReader;

pub fn print_bytes(file_path: &str) {
    let reader = BitReader::new(vec![5, 4, 3]);
    println!("{}", Bit::from(false));
    println!("{}", Bit::from(true));
    println!("{}", file_path);
    // let content = fs::read(file_path).expect("Could not read the file!");
    // println!("{content:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implementation() {
        print_bytes("./data/gunzip.c.gz");
    }
}
