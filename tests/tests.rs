use myfq::samples::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const DATA_DIR: &str = "tests/data";
const SAMPLES_FILE_GOOD: &str = "samples_good.tsv";
const SAMPLES_FILE_EMPTY: &str = "samples_empty.tsv";

#[test]
fn read_sample_table_good() {
    let samples_table_good = Path::new(DATA_DIR).join(SAMPLES_FILE_GOOD);
    let mut samples_file = Box::new(BufReader::new(
        File::open(samples_table_good).expect("Unable to open samples file."),
    )) as Box<dyn BufRead>;
    let samples_table = read_wide_table(samples_file).expect("Unable to open samples table.");
    assert!(samples_table.contains_sample(&PrimerPair {
        forward: "oVK790".to_string(),
        reverse: "oVK791".to_string()
    }));
}

#[test]
fn read_sample_table_empty() {
    let samples_table_empty = Path::new(DATA_DIR).join(SAMPLES_FILE_EMPTY);
    let mut samples_file = Box::new(BufReader::new(
        File::open(samples_table_empty).expect("Unable to open samples file."),
    )) as Box<dyn BufRead>;
    let samples_table = read_wide_table(samples_file);
    match samples_table {
        Ok(_) => panic!("Reading an empty sample table does not return an error."),
        Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
    }
}
