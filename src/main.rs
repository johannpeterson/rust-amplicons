use bio::io::fastq;
use clap::Parser;
use itertools::Itertools;
use myfq::samples::read_wide_table;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use myfq::primers::{read_primer_table, Direction, Primer};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Primers file
    #[arg(short, long)]
    primers: std::path::PathBuf,
    /// Samples file
    #[arg(short, long)]
    samples: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    eprintln!("{:?}", args);

    let mut primer_file = File::open(args.primers).expect("Unable to open primers file.");
    let primer_table = read_primer_table(&mut primer_file).expect("Invalid primers file.");

    let samples_file = Box::new(BufReader::new(
        File::open(args.samples).expect("Unable to open samples file."),
    )) as Box<dyn BufRead>;
    let samples_table = read_wide_table(samples_file).expect("Unable to read samples table.");

    let mut records = fastq::Reader::new(io::stdin()).records();
    let mut writer = fastq::Writer::new(io::stdout());

    let mut records_read = 0;
    let mut records_error = 0;

    let mut matches = HashMap::<String, bool>::with_capacity(primer_table.len() * 2);
    let mut forward_primers = Vec::<&Primer>::with_capacity(primer_table.len());
    let mut reverse_primers = Vec::<&Primer>::with_capacity(primer_table.len());
    for p in &primer_table {
        matches.insert(p.label().to_owned(), false);
        matches.insert(p.label_rc().to_owned(), false);
    }

    while let Some(Ok(record)) = records.next() {
        forward_primers.clear();
        reverse_primers.clear();
        records_read += 1;
        let check = record.check();
        if check.is_err() {
            records_error += 1;
            continue;
        }

        for p in &primer_table {
            matches.insert(p.label().to_owned(), p.is_found_in(record.seq()));
            matches.insert(p.label_rc().to_owned(), p.is_found_in_rc(record.seq()));

            if p.is_found_in(record.seq()) | p.is_found_in_rc(record.seq()) {
                match p.direction() {
                    Direction::Forward => forward_primers.push(p),
                    Direction::Reverse => reverse_primers.push(p),
                }
            }
        }

        let mut primers_string = "".to_string();
        if (forward_primers.len() == 1) & (reverse_primers.len() == 1) {
            let f = forward_primers.pop().map(Primer::label).unwrap_or_default();
            let r = reverse_primers.pop().map(Primer::label).unwrap_or_default();
            primers_string = format!("primers:{f}-{r}");
        } else {
            records_error += 1;
            primers_string = "primers:invalid".to_string();
            // continue;
        }
        // eprintln!("primers_string = '{primers_string}'");

        let match_string: String = matches
            .iter()
            .filter(|(_, v)| **v == true)
            .map(|(k, _)| k)
            .join(":");
        // eprintln!("{match_string}");
        let new_record = fastq::Record::with_attrs(
            record.id(),
            Some(&primers_string),
            record.seq(),
            record.qual(),
        );
        let _ = writer.write_record(&new_record);
    }

    eprintln!("records read: {}\nerrors: {}", records_read, records_error);
}
