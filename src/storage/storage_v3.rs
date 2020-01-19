use crate::storage::*;
use std::error::Error;
use std::io::{Read, Write};

pub struct StorageV3 {}

impl Storage for StorageV3 {
    fn save_result<W: Write>(
        wr: &mut W,
        typing_result: &TypingResult,
    ) -> Result<(), Box<dyn Error>> {
        encode::write_i32(wr, typing_result.correct_words)?;
        encode::write_i32(wr, typing_result.incorrect_words)?;
        encode::write_i32(wr, typing_result.backspaces)?;
        encode::write_i32(wr, typing_result.wpm)?;
        encode::write_u64(wr, typing_result.time)?;
        encode::write_str_len(wr, typing_result.notes.len() as u32)?;
        encode::write_str(wr, &typing_result.notes)?;
        Ok(())
    }

    fn read_result<R: Read>(rd: &mut R) -> Result<TypingResult, Box<dyn Error>> {
        let mut typing_result = TypingResult::default();

        typing_result.correct_words =
            decode::read_i32(rd).map_err(StorageError::MissingCorrectWords)?;
        typing_result.incorrect_words =
            decode::read_i32(rd).map_err(StorageError::MissingIncorrectWords)?;
        typing_result.backspaces = decode::read_i32(rd).map_err(StorageError::MissingBackspaces)?;
        typing_result.wpm = decode::read_i32(rd).map_err(StorageError::MissingWpm)?;
        typing_result.time = decode::read_u64(rd).map_err(StorageError::MissingTime)?;
        let notes_len = decode::read_str_len(rd).map_err(StorageError::MissingNotesLen)?;
        let mut notes = vec![0; notes_len as usize];

        match decode::read_str(rd, &mut notes.as_mut_slice()) {
            Ok(notes_value) => {
                typing_result.notes = String::from(notes_value);
            }
            Err(err) => Err(format!("{:?}", err))?,
        }

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
        notes: String::from("This is a typing result."),
        ..TypingResult::default()
    };

    let _ = StorageV3::save_result(&mut buffer, &typing_result);

    let result = StorageV3::read_result(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(typing_result, result);
}
