use crate::*;
use rmp::*;
use std::error::Error;
use std::io::{ErrorKind, Read, Write};

pub fn save_result<W: Write>(wr: &mut W, test_result: &TypingResult) -> Result<(), Box<dyn Error>> {
    encode::write_i32(wr, test_result.correct_words)?;
    encode::write_i32(wr, test_result.incorrect_words)?;
    encode::write_i32(wr, test_result.backspaces)?;
    encode::write_i32(wr, test_result.wpm)?;
    Ok(())
}

pub fn read_results<R: Read>(rd: &mut R) -> Result<Vec<TypingResult>, Box<dyn Error>> {
    let mut results = Vec::new();

    let mut typing_result = TypingResult::default();

    match decode::read_i32(rd) {
        Err(decode::ValueReadError::InvalidMarkerRead(ref error))
            if error.kind() == ErrorKind::UnexpectedEof => {}
        Err(error) => {
            return Err(Box::new(error));
        }
        Ok(correct_words) => {
            typing_result.correct_words = correct_words;
            typing_result.incorrect_words = decode::read_i32(rd)?;
            typing_result.backspaces = decode::read_i32(rd)?;
            typing_result.wpm = decode::read_i32(rd)?;

            results.push(typing_result);
        }
    }

    Ok(results)
}

#[test]
fn test_read_an_empty_set_of_results() {
    let buffer = Vec::new();

    let all_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(0, all_results.len());
}

#[test]
fn test_write_new_typing_result_to_blank_file_and_read_it_back() {
    let mut buffer = Vec::new();

    let typing_result = TypingResult {
        correct_words: 87,
        incorrect_words: 3,
        backspaces: 2,
        wpm: 87,
    };

    let _ = save_result(&mut buffer, &typing_result);

    let all_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(1, all_results.len());
    assert_eq!(typing_result, all_results[0]);
}
