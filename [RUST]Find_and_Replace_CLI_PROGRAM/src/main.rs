use text::colorizer::*;

#[derive(Debug)]
#[allow(dead_code, unused)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

//The program is taking 4 arguments:
// String to search
// A string to replace with
// Input file
// Output file
fn main() {
    print_help();
}

fn print_help() {
    //FOR ERROR MESSAGES
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    // eprintln!("{} - replace a string with a new string", "Find and Replace");
    eprintln!("Usage: <target strubg> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}