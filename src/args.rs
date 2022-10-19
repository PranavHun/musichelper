use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MusicHelperArgs {
    /// String Instrument Tuning Name
    #[arg(short, long, default_value_t = ("standard").to_string())]
    pub tuning: String,

    #[arg(short, long, default_value_t = ("musichelper.json").to_string())]
    pub datafile: String,

    #[arg(short, long, default_value_t = ("C").to_string())]
    pub key: String,

    #[arg(short, long, default_value_t = ("ionian").to_string())]
    pub scale: String,

    #[arg(short, long, default_value_t = ("maj").to_string())]
    pub chord: String,
}
