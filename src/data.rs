use std::{collections::HashMap, fs::File, io::BufReader};

pub struct NotesData {
    fretboard: Vec<String>,
    pub tunings: HashMap<String, Vec<String>>,
}

impl NotesData {
    pub fn new(mut &self) {
        self.fretboard= vec![
            "A".to_string(), "A#".to_string(),  "B".to_string(),  "C".to_string(),  "C#".to_string(),  "D".to_string(),  "D#".to_string(),  "E".to_string(),  "F".to_string(),  "F#".to_string(),  "G".to_string(),  "G#".to_string(), 
        ];
        let f = File::open("src/tunings.json");
        let reader = BufReader::new(f.unwrap());
    
        self.tunings = serde_json::from_reader(reader).unwrap();
        }
}