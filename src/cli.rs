use clap::Parser;

/// Simple program to analyse a bunch of Zusi result files.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct AnalyseFilesArgs {
    /// Pattern to search for
    #[arg(short, long)]
    pub pattern: String,

    /// Print additional debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}