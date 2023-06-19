use std::{env, process};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};

const SEPARATOR: &str = ";";
const WRONG_FORMATTING: &str = "Output file has wrong formatting";

fn parse_id(line: String) -> Result<u64, &'static str> {
    let last_id: Option<&str> = line.splitn(2,SEPARATOR).next();
    match last_id {
        Some(last_id) => {
            match last_id.parse::<u64>() {
                Ok(last_id) => {
                    Ok(last_id)
                }
                Err(_) => {
                    Err(WRONG_FORMATTING)
                }
            }
        }
        None => {
            Err(WRONG_FORMATTING)
        }
    }
}


// Scans last line in transactions file to get last id
fn get_receipt_id(file: &File) -> Result<u64, &'static str> {
    let reader = BufReader::new(file);

    let last_line = reader.lines().last();

    return match last_line {
        Some(Ok(line)) => parse_id(line),
        Some(Err(_)) => Err("Error occurred while reading file"),
        None => Ok(0)
    }
}

fn open_or_create_file(file_name: &str) -> std::io::Result<std::fs::File> {
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(true);
    options.open(file_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len() - 1;

    if argc < 4 || argc % 2 == 1 {
        eprintln!("Too few or wrong amount of arguments. Use -h or -help");
        process::exit(0);
    }

    if argc == 1 && args[1].contains("h") || args[1].contains("-help") {
        println!("Please enter input in following format:\n\
                 FILE TIME PRODUCT_NAME PRICE [PRODUCT_NAME2 PRICE2]..\n\
                 Where Time should be in format YYYY-DD-MM\n\
                 Note: Arguments in [] are voluntarily");
        process::exit(0);
    }

    let file_path = args[1].clone();

    let open_file = open_or_create_file(&file_path);

    match open_file {
        Err(_) => {
            eprintln!("Error happened while opening file");
            process::exit(0);
        }
        Ok(_) => {
        }
    }

    let file = open_file.unwrap();

    let last_id = get_receipt_id(&file).unwrap_or_else(
        |err| {
            eprintln!("{}", err);
            process::exit(0);
        }
    );
    println!("Last transaction id is {}", last_id);
    process::exit(0);
}
