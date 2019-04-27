use crate::storage::*;
use std::error::Error;
use std::io::{Read, Write};

pub struct StorageV2 {}

impl Storage for StorageV2 {
    fn save_result<W: Write>(
        wr: &mut W,
        typing_result: &TypingResult,
    ) -> Result<(), Box<dyn Error>> {
        encode::write_i32(wr, typing_result.correct_words)?;
        encode::write_i32(wr, typing_result.incorrect_words)?;
        encode::write_i32(wr, typing_result.backspaces)?;
        encode::write_i32(wr, typing_result.wpm)?;
        encode::write_u64(wr, typing_result.time)?;
        Ok(())
    }

    fn read_result<R: Read>(rd: &mut R) -> Result<TypingResult, Box<dyn Error>> {
        let mut typing_result = TypingResult::default();

        typing_result.correct_words = decode::read_i32(rd)?;
        typing_result.incorrect_words = decode::read_i32(rd)?;
        typing_result.backspaces = decode::read_i32(rd)?;
        typing_result.wpm = decode::read_i32(rd)?;
        typing_result.time = decode::read_u64(rd)?;

        Ok(typing_result)
    }
}

#[test]
fn test_write_new_typing_result_to_blank_file_and_read_it_back() {
    let mut buffer = Vec::new();

    let typing_result = TypingResult {
        correct_words: 87,
        incorrect_words: 3,
        backspaces: 2,
        wpm: 87,
        time: 1556223259,
        ..TypingResult::default()
    };

    let _ = StorageV2::save_result(&mut buffer, &typing_result);

    let result = StorageV2::read_result(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(typing_result, result);
}
