use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the input folder
    #[clap(value_parser)]
    pub input: PathBuf,
    /// Path to the output manifest file
    #[clap(value_parser)]
    pub output: PathBuf,

    /// Whitelist paths
    #[clap(short, long, value_parser)]
    pub whitelist: Vec<String>,

    /// Blacklist paths
    #[clap(short, long, value_parser)]
    pub blacklist: Vec<String>,

    /// Version of the patch
    #[clap(short, long, value_parser)]
    pub version: String,

    /// Version on which this patch depends on
    #[clap(short, long, value_parser)]
    pub depends_on: Option<String>,
}
