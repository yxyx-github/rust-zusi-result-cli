use clap::Parser;
use zusi_result_cli::analyse_files;
use zusi_result_cli::cli::AnalyseFilesArgs;

fn main() {
    let args = AnalyseFilesArgs::parse();
    analyse_files(args).unwrap_or_else(|e|
        println!("Error during execution: {e:?}")
    );
}