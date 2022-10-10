use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MusicHelperArgs {
    /// String Instrument Tuning Name
    #[arg(short, long, default_value_t = ("standard").to_string())]
    pub tuning: String,

    #[arg(short = 'f', long, default_value_t = ("tunings.json").to_string())]
    pub tunings_file: String,
}
