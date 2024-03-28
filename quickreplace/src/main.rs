#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

use std::fs;
fn main() {
    let args = parse_args();
    println!("{:?}", args);

    // perform a read operation on a file and store it as data
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            // on error, it will display error messages otherwise it will just fail
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    // performing replace words with match
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // perform a write to a new file from arg.output with the data referenced above
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };
}

use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <targe> <replacement> <INPUT> <OUTPUT>");
}

use std::env;

fn parse_args() -> Arguments {
    // skipping the name of the program being run
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expect 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

// Find and Replace
use regex::Regex;

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    // error type is provided by regex crate
    let regex = Regex::new(target)?;

    // replace_all return a pointer to the orignal text on fail, to avoid extra mem allocation and
    // copying
    // since we want a copy, so to_string is used to geta string no matter what
    Ok(regex.replace_all(text, replacement).to_string())
}
