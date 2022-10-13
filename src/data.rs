use colored::Colorize;
use std::fmt::Debug;
pub(crate) use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Default, Debug)]
pub struct NotesData {
    pub notes: Vec<String>,
    pub tunings: HashMap<String, Vec<String>>,
    pub chords: HashMap<String, Vec<u64>>,
    pub scales: HashMap<String, Vec<u64>>,
}

impl NotesData {
    pub fn new(tuning_file: String) -> Self {
        let data_file = File::open(tuning_file);
        match data_file {
            Ok(data_json_file) => {
                let data_reader = BufReader::new(data_json_file);
                let data_value: serde_json::Value = serde_json::from_reader(data_reader).unwrap();

                // Read Notes from Json
                let mut nts = Vec::<String>::new();
                for note in data_value["notes"].as_array().unwrap() {
                    nts.push(note.as_str().unwrap().to_string());
                }
                // Read Tunings from Json
                let mut tun = HashMap::<String, Vec<String>>::new();
                for (key, val) in data_value["tunings"].as_object().unwrap() {
                    let mut tun_vec = Vec::<String>::new();
                    for tuning in val.as_array().unwrap() {
                        tun_vec.push(tuning.as_str().unwrap().to_string());
                    }
                    tun.insert(key.to_string(), tun_vec);
                }
                // Read Chords from Json
                let mut chrd = HashMap::<String, Vec<u64>>::new();
                for (key, val) in data_value["chords"].as_object().unwrap() {
                    let mut chrd_vec = Vec::<u64>::new();
                    for chord in val.as_array().unwrap() {
                        chrd_vec.push(chord.as_u64().unwrap());
                    }
                    chrd.insert(key.to_string(), chrd_vec);
                }
                // Read Scales from Json
                let mut scl = HashMap::<String, Vec<u64>>::new();
                for (key, val) in data_value["scales"].as_object().unwrap() {
                    let mut scl_vec = Vec::<u64>::new();
                    for scale in val.as_array().unwrap() {
                        scl_vec.push(scale.as_u64().unwrap());
                    }
                    scl.insert(key.to_string(), scl_vec);
                }

                NotesData {
                    notes: nts,
                    tunings: tun,
                    chords: chrd,
                    scales: scl,
                }
            }
            Err(e) => {
                eprintln!(
                    "{} \"{}\" file needs to be provided. \n{}",
                    "ERROR: ".red(),
                    "musichelper.json".red(),
                    e
                );
                std::process::exit(1)
            }
        }
    }
}
