mod args;
mod data;
mod highlights;
mod musichelper;

use args::MusicHelperArgs;
use clap::Parser;
use colored::Colorize;
use data::NotesData;
use highlights::get_highlight;
use itertools::Itertools;

pub(crate) fn main() {
    // Parse CLI args.
    let args = MusicHelperArgs::parse();
    /*     let selected_tuning = args.tuning;
        let selected_data_file: String = args.data_file;
        let selected_key = args.key;
        let selected_
    */
    // Initialize Data from 'tunings.json'
    let data = NotesData::new(args.data_file);
    if data.tunings.contains_key(&args.tuning) {
        let highlight_chord = get_highlight(
            &(data.notes),
            &(args.key),
            data.chords.get(&(args.chord)).unwrap(),
        );
        let highlight_scale = get_highlight(
            &(data.notes),
            &(args.key),
            data.scales.get(&(args.scale)).unwrap(),
        );
        musichelper::print_keyboard(&data, &(args.tuning), &highlight_chord);
        musichelper::print_keyboard(&data, &(args.tuning), &highlight_scale);
    } else {
        eprintln!(
            "{}Tuning named \"{}\" was not found.\nValid Tunings are {:?}",
            "ERROR: ".red().bold(),
            args.tuning.red().bold(),
            data.tunings.keys().sorted()
        );
        std::process::exit(1);
    }
}
