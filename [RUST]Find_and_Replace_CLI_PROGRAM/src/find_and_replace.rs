use text::colorizer::*;
use std::env;
use std::fs;

#[derive(Debug)]
#[allow(dead_code, unused)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    //FOR ERROR MESSAGES
    // eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("{} - replace a string with a new string", "FIND AND REPLACE");
    eprintln!("Usage: <target strubg> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn read_and_write(args: &Arguments) {
    let data: String = match fs::read_to_string(&args.input_file){
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error", args.input_file, e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to output file {}: {:?}", "Error", args.output_file, e);
        }
    }
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!("{} wrong number of arguments, expected 4, got {}", "Error", args.len());
        std::process::exit(1);
    }
    
    Arguments {
        pattern : args[0].clone(),
        replace : args[1].clone(),
        input_file : args[2].clone(),
        output_file : args[3].clone(),
    }

}

pub fn run() {
    let args: Arguments = parse_args();
    read_and_write(&args);
}
