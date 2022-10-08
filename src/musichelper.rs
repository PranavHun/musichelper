use crate::data::NotesData;

/// A Macro to print `fretarr` from the beginning note `key`
macro_rules! print_fretboard_string {
    ($key: expr, $fretarr: ident) => {
        let idx = $fretarr.iter().position(|x| x == &$key).unwrap_or(0);
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
pub fn print_keyboard(notesdata: NotesData, tuning: &str) {
    let mut fb = notesdata.fretboard.clone();
    for note in notesdata.tunings[tuning].clone() {
        print_fretboard_string!(note, fb);
    }
}
