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
    let selected_tuning = args.tuning;
    let selected_tuning_file: String = args.tunings_file;

    // Initialize Data from 'tunings.json'
    let data = NotesData::new(selected_tuning_file);
    if data.tunings.contains_key(&selected_tuning) {
        let highlight = get_highlight(data.fretboard.clone(), "Cmaj7".to_string());

        musichelper::print_keyboard(data, &selected_tuning, highlight);
    } else {
        eprintln!(
            "{}Tuning named \"{}\" was not found.\nValid Tunings are {:?}",
            "ERROR: ".red().bold(),
            selected_tuning.red().bold(),
            data.tunings.keys().sorted()
        );
        std::process::exit(1);
    }
}
