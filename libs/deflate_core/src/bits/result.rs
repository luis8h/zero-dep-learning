
#[derive(Debug, PartialEq, Eq)]
pub struct BitResult {
    pub content: Vec<u32>,
    pub length: usize,
}

impl BitResult {
    pub fn new(content: Vec<u32>, length: usize) -> Self {
        // TODO: handle error logic for invalid length

        BitResult {
            content,
            length
        }
    }
}
