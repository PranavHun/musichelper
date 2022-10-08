mod musichelper;
mod args;
mod data;

use args::MusicHelperArgs;
use clap::Parser;

pub(crate) fn main() {
    let args = MusicHelperArgs::parse();

  

    let selected_tuning = args.tuning;
    
    musichelper::print_keyboard(&selected_tuning);
}
