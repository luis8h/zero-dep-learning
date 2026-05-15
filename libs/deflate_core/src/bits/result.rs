const BUF_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct BitResult {
    content: Vec<u32>,
    length: usize,
}

#[allow(unused)]
#[derive(Debug)]
pub enum BitResultError {
    LengthTooLarge {
        length_provided: usize,
        content_max_length: usize,
    },
    ContentTooLarge {
        content_size_provided: usize,
        content_size_requied: usize,
    },
}

impl TryFrom<(Vec<u32>, usize)> for BitResult {
    type Error = BitResultError;

    fn try_from((content, length): (Vec<u32>, usize)) -> Result<Self, Self::Error> {
        let content_len = content.len() * BUF_SIZE;

        if content_len < length {
            return Err(BitResultError::LengthTooLarge {
                length_provided: length,
                content_max_length: content_len,
            });
        }

        if content_len - BUF_SIZE >= length {
            return Err(BitResultError::ContentTooLarge {
                content_size_provided: content_len,
                content_size_requied: ((length - BUF_SIZE - 1) / BUF_SIZE) * BUF_SIZE,
            });
        }

        Ok(BitResult { content, length })
    }
}
