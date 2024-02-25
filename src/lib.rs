pub mod samples {
    use std::collections::{HashMap, HashSet};
    // use std::error::Error;
    use std::fmt::{self, Write};
    use std::io::{self, BufRead};

    use clap::builder::FalseyValueParser;

    pub struct SampleData {
        name: String,
        is_control: bool,
    }

    #[derive(Eq, PartialEq, Hash)]
    pub struct PrimerPair {
        pub forward: String,
        pub reverse: String,
    }

    type SampleTable = HashMap<PrimerPair, SampleData>;

    pub struct SamplesTable {
        sample_table: HashMap<PrimerPair, SampleData>,
        forward_primers: HashSet<String>,
        reverse_primers: HashSet<String>,
    }

    impl SamplesTable {
        pub fn new() -> SamplesTable {
            SamplesTable {
                sample_table: HashMap::new(),
                forward_primers: HashSet::new(),
                reverse_primers: HashSet::new(),
            }
        }

        pub fn insert(&mut self, primers: PrimerPair, sample: SampleData) -> &mut Self {
            // let forward_primer = primers.forward.clone();
            // let reverse_primer = primers.reverse.clone();
            self.forward_primers.insert(primers.forward.clone());
            self.reverse_primers.insert(primers.reverse.clone());
            self.sample_table.insert(primers, sample);
            self
        }

        pub fn insert_by_names(&mut self, forward: &str, reverse: &str, name: &str) -> &mut Self {
            self.insert(
                PrimerPair {
                    forward: forward.to_string(),
                    reverse: reverse.to_string(),
                },
                SampleData {
                    name: name.to_string(),
                    is_control: false,
                },
            )
        }

        pub fn get(&self, primers: &PrimerPair) -> Option<&SampleData> {
            self.sample_table.get(primers)
        }

        // pub fn write_narrow_table<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        //     out.write_all(b"test")?;
        //     write!(out, "writing samples table: {}", 5)?;
        //     for fwd in &self.forward_primers {
        //         for rev in &self.reverse_primers {
        //             if let Some(sample) = self.sample_table.get(&PrimerPair {
        //                 forward: fwd.to_owned(),
        //                 reverse: rev.to_owned(),
        //             }) {
        //                 write!(out, "{}\t{}\t{}\n", *fwd, *rev, sample.name)?;
        //             }
        //         }
        //     }
        //     out.flush()
        // }
    }

    impl fmt::Display for SamplesTable {
        fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
            if dest.alternate() {
                // todo!("Need to implement alternate format.");
                for (primers, sample) in &self.sample_table {
                    writeln!(
                        dest,
                        "{}\t{}\t{}",
                        primers.forward, primers.reverse, sample.name
                    )?;
                }
            } else {
                write!(dest, "SamplesTable in standard format.")?
            }
            Ok(())
        }
    }

    pub fn fake_samples_table(succeed: bool) -> Result<SampleTable, std::io::Error> {
        if succeed == true {
            let mut samples: SampleTable = SampleTable::new();
            samples.insert(
                PrimerPair {
                    forward: "oVK001".to_string(),
                    reverse: "oVK010".to_string(),
                },
                SampleData {
                    name: "sample 1".to_string(),
                    is_control: false,
                },
            );
            samples.insert(
                PrimerPair {
                    forward: "oVK002".to_string(),
                    reverse: "oVK020".to_string(),
                },
                SampleData {
                    name: "sample 2".to_string(),
                    is_control: false,
                },
            );
            Ok(samples)
        } else {
            Err(std::io::Error::new(
                io::ErrorKind::InvalidData,
                "fake_samples_table succeed=false",
            ))
        }
    }

    pub fn read_wide_table(rdr: Box<dyn BufRead>) -> Result<SampleTable, std::io::Error> {
        // Forward primers are in the first column.  Reverse primers are in the first row.
        let mut fwd_primers: Vec<String> = Vec::new();
        let mut rev_primers: Vec<String> = Vec::new();
        let mut samples_table: SampleTable = HashMap::new();
        let mut lines = rdr
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.chars().all(|c| c.is_whitespace())) // ignore blank lines
            .filter(|l| !l.trim_start().starts_with('#')); // ignore comment lines

        if let Some(line) = lines.next() {
            if !line.starts_with([' ', '\t']) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "First line must begin with whitespace.",
                ));
            }
            rev_primers.extend(line.split_ascii_whitespace().map(|s| s.to_string()));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File must contain at least one line.",
            ));
        }

        while let Some(line) = lines.next() {
            let mut elements = line.split_ascii_whitespace();
            if let Some(fp) = elements.next() {
                let mut column = 0;
                while let Some(sample) = elements.next() {
                    let rp = rev_primers[column].clone();
                    samples_table.insert(
                        PrimerPair {
                            forward: fp.to_string(),
                            reverse: rp.to_string(),
                        },
                        SampleData {
                            name: sample.to_string(),
                            is_control: false,
                        },
                    );
                    column += 1;
                }
                fwd_primers.push(fp.to_string());
            }
        }

        Ok(samples_table)
    }

    // pub fn read_samples_table<R: Read>(rdr: &mut R) -> Result<SampleTable, std::io::Error> {
    //     let mut samples: SampleTable = HashMap::new();
    //     let mut contents = String::new();
    //     rdr.read_to_string(&mut contents)?;
    //     let mut lines = contents.lines();
    //     let header = lines.next().ok_or(std::io::Error::new(
    //         io::ErrorKind::InvalidData,
    //         "Samples file must have at least one line.",
    //     ))?;
    //     header.starts_with([' ', '\t']);
    //     let fwd_primers: Vec<&str> = header.split_ascii_whitespace().collect();
    //     let rev_primers: Vec<String> = Vec::new();
    //     while let Some(row) = lines.next() {
    //         let mut row_items = row.split_ascii_whitespace();

    //         let rev_primer = {
    //             if let Some(rev_primer) = row_items.next() {
    //                 // add sample names from this row here
    //                 rev_primer
    //             } else {
    //                 continue; // skip this empty row
    //             }
    //         };

    //         while let Some((i, sample_name)) = row_items.enumerate().next() {
    //             samples.insert(
    //                 (fwd_primers[i].to_string(), rev_primer.to_string()),
    //                 SampleData {
    //                     name: sample_name.to_string(),
    //                     is_control: false,
    //                 },
    //             );
    //         }
    //     }
    //     //Ok(samples)
    //     fake_samples_table(true)
    // }

    #[test]
    fn create_samples_table() {
        let mut t: SamplesTable = SamplesTable::new();
        t.insert(
            PrimerPair {
                forward: "p001".to_string(),
                reverse: "p010".to_string(),
            },
            SampleData {
                name: "sample_1".to_string(),
                is_control: false,
            },
        );
    }

    #[test]
    fn add_sample_by_names() {
        let mut t: SamplesTable = SamplesTable::new();
        t.insert_by_names("p001", "p010", "sample_1");
    }

    #[test]
    fn write_wide_table() {
        let mut t: SamplesTable = SamplesTable::new();
        t.insert(
            PrimerPair {
                forward: "p001".to_string(),
                reverse: "p010".to_string(),
            },
            SampleData {
                name: "sample_1".to_string(),
                is_control: false,
            },
        );

        let mut s: String = String::new();
        write!(s, "{}", t);
        s.find("p001").expect("fwd not found");
        s.find("p010").expect("rev not found");
        s.find("sample_1").expect("name not found");
    }

    #[test]
    fn write_narrow_table() {
        let mut t: SamplesTable = SamplesTable::new();
        t.insert(
            PrimerPair {
                forward: "p001".to_string(),
                reverse: "p010".to_string(),
            },
            SampleData {
                name: "sample_1".to_string(),
                is_control: false,
            },
        );

        let mut s: String = String::new();
        write!(s, "{:#}", t);
        s.find("p001").expect("fwd not found");
        s.find("p010").expect("rev not found");
        s.find("sample_1").expect("name not found");
    }
}

pub mod primers {

    use bio::alphabets::dna;
    use bio::pattern_matching::bom::BOM;
    use serde::Deserialize;
    use std::io::Read;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct PrimerRecord {
        label: String,
        sequence: String,
        barcode: String,
        direction: String,
    }

    type PrimerTable = Vec<Primer>;

    pub fn read_primer_table<R: Read>(
        rdr: &mut R,
    ) -> Result<PrimerTable, Box<dyn std::error::Error>> {
        let mut primer_table = PrimerTable::new();
        let mut primer_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(rdr);
        for result in primer_reader.deserialize() {
            let primer_record: PrimerRecord = result.expect("Bad primer record.");
            let primer = Primer::new(
                primer_record.label.as_str(),
                &primer_record.sequence.into_bytes(),
                &primer_record.barcode.into_bytes(),
                if primer_record.direction == "F" {
                    Direction::Forward
                } else {
                    Direction::Reverse
                },
            );
            match primer.check() {
                Err(e) => eprintln!("Primer read error: {e}"),
                Ok(_) => primer_table.push(primer),
            }
        }

        Ok(primer_table)
    }

    /// Encodes the reading direction for Primer (Forward or Reverse).
    ///
    /// # Examples
    ///
    /// ```
    /// use myfq::primers::*;
    /// let d = Direction::Forward;
    /// assert_eq!(d.opposite(), Direction::Reverse);
    /// ```
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Direction {
        Forward,
        Reverse,
    }

    impl Direction {
        pub fn opposite(&self) -> Direction {
            match self {
                Direction::Forward => Direction::Reverse,
                Direction::Reverse => Direction::Forward,
            }
        }
    }

    /// Data structure to hold a single primer.
    ///
    /// # Examples
    ///
    /// ```
    /// use myfq::primers::*;
    /// let seq = b"ACTGACTGACTG";
    /// let barcode = b"GACT";
    /// let p = Primer::new("PrimerLabel", seq, barcode, Direction::Forward);
    /// assert!(p.check() == Ok(()));
    /// ```
    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Primer {
        label: String,
        label_rc: String,
        sequence: Vec<u8>,
        sequence_rc: Vec<u8>,
        barcode: Vec<u8>,
        direction: Direction,
        searcher: BOM,
        searcher_rc: BOM,
    }

    impl Primer {
        /// Create a new Primer struct.
        ///
        /// The reverse complement sequence is computed.
        /// [bio::pattern_matching::bom::BOM](https://docs.rs/bio/latest/bio/pattern_matching/bom/struct.BOM.html) objects are created to search for the
        /// primer sequence (both forward and reverse complement).
        pub fn new(label: &str, sequence: &[u8], barcode: &[u8], direction: Direction) -> Primer {
            Self {
                label: label.to_owned(),
                label_rc: format!("{label}rc"),
                sequence: sequence.to_vec(),
                barcode: barcode.to_vec(),
                direction,
                sequence_rc: dna::revcomp(sequence.to_owned()),
                searcher: BOM::new(sequence.to_owned()),
                searcher_rc: BOM::new(dna::revcomp(sequence.to_owned())),
            }
        }

        /// Check the validity of the Primer object after creation.
        ///
        /// The label must not be empty & the sequence must be consist of valid DNA
        /// bases, as defined in [bio::alphabets::dna](https://docs.rs/bio/latest/bio/alphabets/dna/index.html).
        pub fn check(&self) -> Result<(), &str> {
            // primer must have a non-empty label
            if self.label.is_empty() {
                return Err("Empty primer label.");
            }
            // primer sequence must be valid DNA characters
            if !dna::alphabet().is_word(self.sequence.to_owned()) {
                return Err("Invalid DNA sequence.");
            }
            Ok(())
        }

        /// Search in `seq` for the primer sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use myfq::primers::*;
        /// let seq = b"AAAA";
        /// let bc = b"AA";
        /// let p = Primer::new("primer", seq, bc, Direction::Forward);
        ///
        /// let sequence1 = b"GGGGGGAAAAGGGGG";
        /// let sequence2 = b"GGGGGGGGGGGGGGG";
        ///
        /// assert!(p.is_found_in(sequence1));
        /// assert!(!p.is_found_in(sequence2));
        /// ```

        pub fn is_found_in(&self, seq: &[u8]) -> bool {
            !self.searcher.find_all(seq).next().is_none()
        }

        /// Search in `seq` for the reverse complement of the primer sequence.
        pub fn is_found_in_rc(&self, seq: &[u8]) -> bool {
            !self.searcher_rc.find_all(seq).next().is_none()
        }

        /// Returns the primer label.
        ///
        /// This is the string passed to `new()` as `label`.
        pub fn label(&self) -> &str {
            &self.label
        }

        /// Returns a label for the reverse complement of the primer.
        ///
        /// Computed as `label` followed by "rc".
        pub fn label_rc(&self) -> &str {
            &self.label_rc
        }

        /// Returns the direction of the primer object.
        pub fn direction(&self) -> Direction {
            self.direction
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn can_create_a_primer() {
            let seq = b"ACTGACTGACTG";
            let bc = b"GACT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            assert!(p.check() == Ok(()));
        }

        #[test]
        fn invalid_primer_sequence() {
            let seq = b"ACTGAXTGACTG";
            let bc = b"GACT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            assert!(p.check().is_err());
        }

        #[test]
        fn primer_sequence_found() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            let sequence = b"ACTGACTGAGATACAGACTGACTGACTGACTGACTGACTGACTG";
            assert!(p.is_found_in(sequence));
        }

        #[test]
        fn primer_rc_sequence_found() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            let sequence = b"ACTGACTGACTGACTGTATCTGACTGACTGACTGACTGACTG";
            assert!(p.is_found_in_rc(sequence));
        }

        #[test]
        fn primer_sequence_not_found() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            let sequence = b"ACTGACTGACTGACTGACTGACTGACTGACTGACTG";
            assert!(!p.is_found_in(sequence));
        }

        #[test]
        fn primer_sequence_front_truncated() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            let sequence = b"ATACAACTGACTGACTGACTGACTGACTGACTGACTGACTG";
            assert!(!p.is_found_in(sequence));
        }

        #[test]
        fn primer_sequence_end_truncated() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            let sequence = b"ACTGACTGACTGACTGACTGACTGACTGACTGACTGGATAC";
            assert!(!p.is_found_in(sequence));
        }

        #[test]
        fn retrieve_primer_labels() {
            let seq = b"GATACA";
            let bc = b"GAT";
            let p = Primer::new("primer", seq, bc, Direction::Forward);
            assert_eq!(p.label(), "primer");
            assert_eq!(p.label_rc(), "primerrc");
        }

        #[test]
        fn test_direction() {
            let d = Direction::Forward;
            assert_eq!(d, Direction::Forward);
            assert_eq!(d.opposite(), Direction::Reverse);
        }
    }
}
