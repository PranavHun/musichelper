use colored::Colorize;
use std::fmt::Debug;
pub(crate) use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Default, Debug)]
pub struct NotesData {
    pub fretboard: Vec<String>,
    pub tunings: HashMap<String, Vec<String>>,
}

impl NotesData {
    pub fn new(tuning_file: String) -> Self {
        let fb = vec![
            "A".to_string(),
            "A#".to_string(),
            "B".to_string(),
            "C".to_string(),
            "C#".to_string(),
            "D".to_string(),
            "D#".to_string(),
            "E".to_string(),
            "F".to_string(),
            "F#".to_string(),
            "G".to_string(),
            "G#".to_string(),
        ];
        let f = File::open(tuning_file);
        match f {
            Ok(tunings_json_file) => {
                let reader = BufReader::new(tunings_json_file);
                let tun = serde_json::from_reader(reader).unwrap();
                NotesData {
                    fretboard: fb,
                    tunings: tun,
                }
            }
            Err(e) => {
                eprintln!(
                    "{} \"{}\" file needs to be provided. \n{}",
                    "ERROR: ".red(),
                    "tunings.json".red(),
                    e
                );
                std::process::exit(1)
            }
        }
    }
}
