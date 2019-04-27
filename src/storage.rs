use crate::*;
use dirs::*;
use lazy_static::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rmp::*;
use std::error::Error;
use std::fs::{DirBuilder, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
mod storage_v1;
mod storage_v2;

#[repr(i8)]
#[derive(FromPrimitive)]
enum StorageVersions {
    V1 = 0x01,
    V2 = 0x02,
}

pub const CURRENT_VERSION: i8 = StorageVersions::V2 as i8;

lazy_static! {
    pub static ref RESULTS_PATH: PathBuf =
        config_dir().unwrap().join("wpm").join("typing_results.wpm");
}

#[derive(Default, Debug)]
pub struct ReadTypingResults {
    results: Vec<TypingResult>,
    records_need_upgrading: bool, // If older versions were read, we need to save them back
}

fn read_results<R: Read>(rd: &mut R) -> Result<ReadTypingResults, Box<dyn Error>> {
    let mut read_typing_results = ReadTypingResults::default();

    loop {
        match decode::read_marker(rd) {
            Err(_) => break,
            Ok(Marker::FixExt1) => match decode::read_data_i8(rd) {
                Ok(version_num) => {
                    if version_num < CURRENT_VERSION {
                        read_typing_results.records_need_upgrading = true;
                    }
                    let typing_result = match FromPrimitive::from_i8(version_num) {
                        Some(StorageVersions::V1) => Some(storage_v1::StorageV1::read_result(rd)?),
                        Some(StorageVersions::V2) => Some(storage_v2::StorageV2::read_result(rd)?),
                        None => None,
                    };
                    if let Some(typing_result) = typing_result {
                        read_typing_results.results.push(typing_result);
                    }
                }
                Err(decode::ValueReadError::InvalidMarkerRead(ref error))
                    if error.kind() == ErrorKind::UnexpectedEof =>
                {
                    break
                }
                Err(error) => return Err(error.into()),
            },
            Ok(_) => {}
        }
    }
    Ok(read_typing_results)
}

pub fn read_results_from_file() -> Result<ReadTypingResults, Box<dyn Error>> {
    match OpenOptions::new().read(true).open(RESULTS_PATH.as_path()) {
        Err(ref error) if error.kind() == ErrorKind::NotFound => Ok(ReadTypingResults::default()),
        Err(error) => Err(error.into()),
        Ok(mut fd) => read_results(&mut fd),
    }
}

fn save_result<W: Write>(wr: &mut W, typing_result: &TypingResult) -> Result<(), Box<dyn Error>> {
    encode::write_ext_meta(wr, 1, CURRENT_VERSION)?;
    storage_v2::StorageV2::save_result(wr, typing_result)
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

pub trait Storage {
    fn save_result<W: Write>(
        wr: &mut W,
        typing_result: &TypingResult,
    ) -> Result<(), Box<dyn Error>>;
    fn read_result<R: Read>(rd: &mut R) -> Result<TypingResult, Box<dyn Error>>;
}

#[test]
fn test_read_an_empty_set_of_results() {
    let buffer = Vec::new();

    let all_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(0, all_results.results.len());
    assert_eq!(false, all_results.records_need_upgrading);
}

#[test]
fn test_write_then_read_back_v1_records_only() {
    let typing_result = TypingResult {
        correct_words: 87,
        incorrect_words: 3,
        backspaces: 2,
        wpm: 87,
        ..TypingResult::default()
    };

    let mut buffer = Vec::new();

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V1 as i8).unwrap();
    storage_v1::StorageV1::save_result(&mut buffer, &typing_result).unwrap();

    let read_typing_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(1, read_typing_results.results.len());
    assert_eq!(typing_result, read_typing_results.results[0]);
    assert_eq!(true, read_typing_results.records_need_upgrading);
}

#[test]
fn test_write_then_read_back_v2_records_only() {
    let typing_result = TypingResult {
        correct_words: 87,
        incorrect_words: 3,
        backspaces: 2,
        wpm: 87,
        time: 1556223259,
        ..TypingResult::default()
    };

    let mut buffer = Vec::new();

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V2 as i8).unwrap();
    storage_v2::StorageV2::save_result(&mut buffer, &typing_result).unwrap();

    let read_typing_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(1, read_typing_results.results.len());
    assert_eq!(typing_result, read_typing_results.results[0]);
    assert_eq!(false, read_typing_results.records_need_upgrading);
}

#[test]
fn test_write_a_future_version_which_will_be_ignored_when_read_back() {
    let typing_result = TypingResult {
        correct_words: 87,
        incorrect_words: 3,
        backspaces: 2,
        wpm: 87,
        time: 1556223259,
        ..TypingResult::default()
    };

    let mut buffer = Vec::new();

    encode::write_ext_meta(&mut buffer, 1, std::i8::MAX).unwrap();
    storage_v2::StorageV2::save_result(&mut buffer, &typing_result).unwrap();

    let read_typing_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(0, read_typing_results.results.len());
    assert_eq!(false, read_typing_results.records_need_upgrading);
}

#[test]
fn test_write_some_v1s_and_v2s_and_read_them_back() {
    let mut buffer = Vec::new();

    let typing_result1 = TypingResult {
        correct_words: 1,
        incorrect_words: 2,
        backspaces: 3,
        wpm: 1,
        ..TypingResult::default()
    };

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V1 as i8).unwrap();
    storage_v1::StorageV1::save_result(&mut buffer, &typing_result1).unwrap();

    let typing_result2 = TypingResult {
        correct_words: 2,
        incorrect_words: 2,
        backspaces: 3,
        wpm: 2,
        ..TypingResult::default()
    };

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V1 as i8).unwrap();
    storage_v1::StorageV1::save_result(&mut buffer, &typing_result2).unwrap();

    let typing_result3 = TypingResult {
        correct_words: 3,
        incorrect_words: 2,
        backspaces: 3,
        wpm: 3,
        time: 1556223259,
        ..TypingResult::default()
    };

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V2 as i8).unwrap();
    storage_v2::StorageV2::save_result(&mut buffer, &typing_result3).unwrap();

    let typing_result4 = TypingResult {
        correct_words: 4,
        incorrect_words: 2,
        backspaces: 3,
        wpm: 4,
        time: 1556223265,
        ..TypingResult::default()
    };

    encode::write_ext_meta(&mut buffer, 1, StorageVersions::V2 as i8).unwrap();
    storage_v2::StorageV2::save_result(&mut buffer, &typing_result4).unwrap();

    let read_typing_results = read_results(&mut &buffer[..]).expect("Read back the results");

    assert_eq!(4, read_typing_results.results.len());
    assert_eq!(
        vec![
            typing_result1,
            typing_result2,
            typing_result3,
            typing_result4
        ],
        read_typing_results.results
    );
    assert_eq!(true, read_typing_results.records_need_upgrading);
}
