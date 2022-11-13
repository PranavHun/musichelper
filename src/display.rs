use crate::data::NotesData;
use colored::Colorize;

/// A Macro to print `fretarr` from the beginning note `key`
macro_rules! print_fretboard_string {
    ($key: expr, $fretarr: ident, $highlight: ident) => {
        let idx = $fretarr.iter().position(|x| x == &$key).unwrap_or(0);
        if idx > 0 {
            $fretarr.rotate_left(idx);
        }

        for _ in [0, 1] {
            for note in &$fretarr {
                match $highlight.iter().find(|&s| s == note) {
                    None => print!("{:4}", note),
                    _ => print!("{:4}", note.red()),
                }
            }
        }
        match $highlight.iter().find(|&s| s == &$key) {
            None => print!("{:4}", $key),
            _ => print!("{:4}", $key.red()),
        }
        println!();
    };
}

///A function to define the `fretboard` and `tunings`,
///and print the fretboard for the given `tuning`.
pub fn print_fretboard(notesdata: &NotesData, tuning: &str, highlight: &[String]) {
    let mut fb = notesdata.notes.clone();

    for fret_no in 0..=(fb.len() * 2) {
        print!("{:<4}", fret_no);
    }
    println!();

    for _ in 0..=(fb.len() * 2) {
        print!("----");
    }
    println!();

    for note in notesdata.tunings[tuning].clone() {
        print_fretboard_string!(note, fb, highlight);
    }

    for _ in 0..=(fb.len() * 2) {
        print!("----");
    }
    println!();

    for fret_no in 0..=(fb.len() * 2) {
        print!("{:<4}", fret_no);
    }
    println!();
    println!();
}
