pub(crate) use std::{collections::HashMap, fs::File, io::BufReader};
use std::{fmt::Debug, io::Read, result::Result};

use serde_json::Value;

#[derive(Default, Debug)]
pub struct NotesData {
    pub notes: Vec<String>,
    pub tunings: HashMap<String, Vec<String>>,
    pub chords: HashMap<String, Vec<u64>>,
    pub scales: HashMap<String, Vec<u64>>,
}

impl NotesData {
    pub fn new(tuning_file: String) -> Option<Self> {
        let data_file_check = File::open(tuning_file);
        let mut filestr = "".to_string();
        let default_filestr = include_str!("default_data_file.json").to_string();
        match data_file_check {
            Ok(data_file) => {
                let _ = BufReader::new(data_file)
                    .read_to_string(&mut filestr)
                    .unwrap();
            }
            Err(e) => {
                eprintln!("ERROR: {}.\nUsing default datafile.", e);
                filestr = default_filestr;
            }
        }
        let data_value_check: Result<Value, serde_json::Error> =
            serde_json::from_str(filestr.as_str());

        match data_value_check {
            Ok(data_value) => {
                // Read Notes from Json
                let mut nts = Vec::<String>::new();

                for note in data_value["notes"].as_array()? {
                    nts.push(note.as_str()?.to_string());
                }
                // Read Tunings from Json
                let mut tun = HashMap::<String, Vec<String>>::new();
                for (key, val) in data_value["tunings"].as_object()? {
                    let mut tun_vec = Vec::<String>::new();
                    for tuning in val.as_array()? {
                        tun_vec.push(tuning.as_str()?.to_string());
                    }
                    tun.insert(key.to_string(), tun_vec);
                }
                // Read Chords from Json
                let mut chrd = HashMap::<String, Vec<u64>>::new();
                for (key, val) in data_value["chords"].as_object()? {
                    let mut chrd_vec = Vec::<u64>::new();
                    for chord in val.as_array()? {
                        chrd_vec.push(chord.as_u64()?);
                    }
                    chrd.insert(key.to_string(), chrd_vec);
                }
                // Read Scales from Json
                let mut scl = HashMap::<String, Vec<u64>>::new();
                for (key, val) in data_value["scales"].as_object()? {
                    let mut scl_vec = Vec::<u64>::new();
                    for scale in val.as_array()? {
                        scl_vec.push(scale.as_u64()?);
                    }
                    scl.insert(key.to_string(), scl_vec);
                }

                Some(NotesData {
                    notes: nts,
                    tunings: tun,
                    chords: chrd,
                    scales: scl,
                })
            }
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }
}
