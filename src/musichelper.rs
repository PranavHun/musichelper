use std::{collections::HashMap, fs::File, io::BufReader};

/// A Macro to print `fretarr` from the beginning note `key`
macro_rules! print_fretboard_string {
    ($key: expr, $fretarr: ident) => {
        let idx = $fretarr.iter().position(|&x| x == $key).unwrap_or(0);
        if idx > 0 {
            $fretarr.rotate_left(idx);
        }

        for _ in [0, 1] {
            for note in &$fretarr {
                print!("{:4} ", note);
            }
        }

        println!("{}", $key);
    };
}

///A function to define the `fretboard` and `tunings`,
///and print the fretboard for the given `tuning`.
pub fn print_keyboard(tuning: &str) {
    let mut fretboard = vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];

    let f = File::open("src/tunings.json");
    let reader = BufReader::new(f.unwrap());

    let tunings: HashMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();

    for note in tunings[tuning].clone() {
        print_fretboard_string!(note, fretboard);
    }
}
