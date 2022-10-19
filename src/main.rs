mod args;
mod data;
mod display;
mod highlights;

use args::MusicHelperArgs;
use clap::Parser;

use data::NotesData;
use highlights::get_highlight;

pub(crate) fn main() {
    // Parse CLI args.
    let args = MusicHelperArgs::parse();

    // Initialize Data from 'tunings.json'
    let data_check = NotesData::new(args.datafile);
    match data_check {
        Some(data) => {
            if data.tunings.contains_key(&args.tuning) {
                if data.chords.contains_key(&args.chord) {
                    let highlight_chord = get_highlight(
                        &(data.notes),
                        &(args.key),
                        data.chords.get(&(args.chord)).unwrap(),
                    );
                    display::print_keyboard(&data, &(args.tuning), &highlight_chord);
                } else {
                    eprintln!("ERROR: Invalid Chord argument given: {}", args.chord);
                }
                if data.scales.contains_key(&args.scale) {
                    let highlight_scale = get_highlight(
                        &(data.notes),
                        &(args.key),
                        data.scales.get(&(args.scale)).unwrap(),
                    );
                    display::print_keyboard(&data, &(args.tuning), &highlight_scale);
                } else {
                    eprintln!("ERROR: Invalid Scale argument given: {}", args.scale);
                }
            } else {
                eprintln!("ERROR: Invalid Tuning argument given: {}", args.tuning);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("ERROR: Unable to parse data file");
            std::process::exit(1)
        }
    }
}
