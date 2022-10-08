use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MusicHelperArgs {
        /// String Instrument Tuning Name
        #[arg(short, long, default_value_t = ("std").to_string())]
        pub tuning : String, 
    }
