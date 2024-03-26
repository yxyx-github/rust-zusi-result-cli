use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use glob::{glob, PatternError};
use zusi_result_lib::result_analyser_group::ResultAnalyserGroup;
use zusi_xml_lib::xml::zusi::{DeError, Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

pub struct AnalyseFilesArgs {
    pub pattern: String,
    pub debug: bool,
}

pub fn analyse_files(args: AnalyseFilesArgs) -> Result<(), PatternError> {
    println!("Analyse files by pattern: {}", args.pattern);

    let mut results: Vec<ZusiResult> = vec![];

    for entry in glob(&args.pattern)? {
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
    print_analysis(results);
    Ok(())
}

#[derive(Debug)]
enum ReadResultError {
    IOError(io::Error),
    DeError(DeError),
    NoResults,
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
    Err(ReadResultError::NoResults)
}

fn print_analysis(results: Vec<ZusiResult>) {
    let mut analyser_group: ResultAnalyserGroup = results.try_into().unwrap();
    println!("total distance: {} m", analyser_group.total_distance().unwrap());
    println!("average distance: {} m", analyser_group.average_distance().unwrap());
    let average_speed = analyser_group.average_speed().unwrap();
    println!("average speed: {} m/s = {} km/h", average_speed, average_speed * 3.6);
    let pure_average_speed = analyser_group.pure_average_speed().unwrap();
    println!("pure average speed: {} m/s = {} km/h", pure_average_speed, pure_average_speed * 3.6);
    println!("total driving time: {}", analyser_group.total_driving_time().unwrap());
    println!("total pure driving time: {}", analyser_group.total_pure_driving_time().unwrap());
}