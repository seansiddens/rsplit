use std::{env, fs::File, io::Read};

// Print program usage.
fn usage() {
    println!("usage: ./split: <split_pattern> [<file1> <file2> ...]");
}

// Split a reader by the delimiter.
fn split_file<R: Read>(reader: &mut R, delim: &str)
{
    // Read file into string.
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read file to string.\n");

    // Split file by delimiter and print.
    let output = input.replace(delim, "\n");
    print!("{}", output);
}

fn main() {
    // Check if enough arguments are supplied.
    if env::args().len() <= 2 {
        usage();
        return;
    }

    // Store commandline arguments.
    let mut args = Vec::new();
    for arg in env::args().skip(1) {
        args.push(arg);
    }

    let delimiter = args[0].clone();

    // Process files.
    for i in 1..args.len() {
        if &args[i] == "-" {
            split_file(&mut std::io::stdin(), &delimiter);
        } else {
            let mut file = File::open(&args[i]).expect("Failed to open file for reading!\n");
            split_file(&mut file, &delimiter);
        }
    }
}
