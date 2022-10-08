mod args;
mod data;
mod musichelper;

use colored::Colorize;

use args::MusicHelperArgs;
use clap::Parser;
use data::NotesData;

pub(crate) fn main() {
    // Parse CLI args.
    let args = MusicHelperArgs::parse();
    let selected_tuning = args.tuning;

    // Initialize Data from 'tunings.json'
    let data = NotesData::new();
    if data.tunings.contains_key(&selected_tuning) {
        musichelper::print_keyboard(data, &selected_tuning);
    } else {
        eprintln!(
            "{}Tuning named \"{}\" was not found.\nValid Tunings are {:?}",
            "ERROR: ".red().bold(),
            selected_tuning.red().bold(),
            data.tunings.keys()
        );
        std::process::exit(1);
    }
}
