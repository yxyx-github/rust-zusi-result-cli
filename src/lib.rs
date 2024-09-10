pub mod cli;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use glob::{glob, PatternError};
use zusi_result_lib::result_analyser::{AnalyseError, ResultAnalyser};
use zusi_result_lib::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};
use zusi_xml_lib::xml::zusi::{DeError, Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use crate::cli::AnalyseFilesArgs;

#[derive(Debug)]
pub enum AnalyseFilesError {
    PatternError(PatternError),
    PrintAnalysisError(PrintAnalysisError),
}

pub fn analyse_files(args: AnalyseFilesArgs) -> Result<(), AnalyseFilesError> {
    println!("Analyse files by pattern: {}", args.pattern);

    let mut results: Vec<ZusiResult> = vec![];

    for entry in glob(&args.pattern).map_err(|e| AnalyseFilesError::PatternError(e))? {
        match entry {
            Ok(path) => {
                match read_result(&path) {
                    Ok(result) => {
                        if args.debug {
                            println!("{:?}", path.display())
                        }
                        results.push(result);
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);

            }
        }
    }

    println!();
    println!("Analysis results:");
    print_analysis(results).map_err(|e| AnalyseFilesError::PrintAnalysisError(e))?;
    Ok(())
}

#[derive(Debug)]
enum ReadResultError {
    IOError(io::Error),
    DeError(DeError),
    NoResult,
}

fn read_result(path: &PathBuf) -> Result<ZusiResult, ReadResultError> {
    let mut input_file = File::open(path).map_err(|err| ReadResultError::IOError(err))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).map_err(|err| ReadResultError::IOError(err))?;
    let zusi = Zusi::from_xml(&contents).map_err(|err| ReadResultError::DeError(err))?;
    for value in zusi.value {
        if let ZusiValue::Result(result) = value {
            return Ok(result);
        }
    }
    Err(ReadResultError::NoResult)
}

#[derive(Debug)]
pub enum PrintAnalysisError {
    CreateAnalyserGroupError(CreateAnalyserGroupError),
    AnalyseError(AnalyseError),
}

fn print_analysis(results: Vec<ZusiResult>) -> Result<(), PrintAnalysisError> {
    let mut analyser_group: ResultAnalyserGroup<ResultAnalyser<ZusiResult>, ZusiResult> = results.try_into().map_err(|e| PrintAnalysisError::CreateAnalyserGroupError(e))?;
    println!("total distance: {} m", analyser_group.total_distance().map_err(|e| PrintAnalysisError::AnalyseError(e))?);
    println!("average distance: {} m", analyser_group.average_distance().map_err(|e| PrintAnalysisError::AnalyseError(e))?);
    let average_speed = analyser_group.average_speed().map_err(|e| PrintAnalysisError::AnalyseError(e))?;
    println!("average speed: {} m/s = {} km/h", average_speed, average_speed * 3.6);
    let pure_average_speed = analyser_group.pure_average_speed().map_err(|e| PrintAnalysisError::AnalyseError(e))?;
    println!("pure average speed: {} m/s = {} km/h", pure_average_speed, pure_average_speed * 3.6);
    println!("total driving time: {}", analyser_group.total_driving_time().map_err(|e| PrintAnalysisError::AnalyseError(e))?);
    println!("total pure driving time: {}", analyser_group.total_pure_driving_time().map_err(|e| PrintAnalysisError::AnalyseError(e))?);
    Ok(())
}
