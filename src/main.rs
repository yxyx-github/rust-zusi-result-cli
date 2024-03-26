use zusi_result_cli::{analyse_files, AnalyseFilesArgs};

fn main() {
    let args = AnalyseFilesArgs {
        pattern: "../zusi-result-lib/data/*.result.xml".into(),
        debug: true,
    };
    analyse_files(args).unwrap_or_else(|e|
        println!("Error during execution: {e}")
    );
}