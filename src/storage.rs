use crate::*;
use dirs::*;
use lazy_static::*;
use rmp::*;
use std::error::Error;
use std::fs::{DirBuilder, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

#[repr(i8)]
enum StorageVersions {
    V1 = 0x01,
}

pub const CURRENT_VERSION: i8 = StorageVersions::V1 as i8;

lazy_static! {
    pub static ref RESULTS_PATH: PathBuf =
        config_dir().unwrap().join("wpm").join("typing_results.wpm");
}

pub fn save_result_to_file(typing_result: &TypingResult) -> Result<(), Box<dyn Error>> {
    if let Some(dir_name) = RESULTS_PATH.parent() {
        DirBuilder::new().recursive(true).create(dir_name)?;
    }
    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(RESULTS_PATH.as_path())?;
    save_result(&mut fd, typing_result)
}

pub fn save_result<W: Write>(wr: &mut W, test_result: &TypingResult) -> Result<(), Box<dyn Error>> {
    encode::write_ext_meta(wr, 1, CURRENT_VERSION)?;
    encode::write_i32(wr, test_result.correct_words)?;
    encode::write_i32(wr, test_result.incorrect_words)?;
    encode::write_i32(wr, test_result.backspaces)?;
    encode::write_i32(wr, test_result.wpm)?;
    Ok(())
}

pub fn read_results<R: Read>(rd: &mut R) -> Result<Vec<TypingResult>, Box<dyn Error>> {
    let mut results = Vec::new();

    loop {
        match decode::read_ext_meta(rd) {
            Err(decode::ValueReadError::InvalidMarkerRead(ref error))
                if error.kind() == ErrorKind::UnexpectedEof =>
            {
                break
            }
            Err(error) => {
                return Err(Box::new(error));
            }
            Ok(decode::ExtMeta {
                typeid: _version_num,
                ..
            }) => {
                let mut typing_result = TypingResult::default();

                typing_result.correct_words = decode::read_i32(rd)?;
                typing_result.incorrect_words = decode::read_i32(rd)?;
                typing_result.backspaces = decode::read_i32(rd)?;
                typing_result.wpm = decode::read_i32(rd)?;

                results.push(typing_result);
            }
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

#[test]
fn test_write_multiple_typing_results_to_blank_file_and_read_them_all_back() {
    let mut buffer = Vec::new();

    let typing_results = vec![
        TypingResult {
            correct_words: 87,
            incorrect_words: 3,
            backspaces: 2,
            wpm: 87,
        },
        TypingResult {
            correct_words: 15,
            incorrect_words: 0,
            backspaces: 25,
            wpm: 15,
        },
        TypingResult {
            correct_words: 125,
            incorrect_words: 65,
            backspaces: 7,
            wpm: 125,
        },
    ];

    for typing_result in &typing_results {
        let _ = save_result(&mut buffer, typing_result);
    }

    let all_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(typing_results, all_results);
}
