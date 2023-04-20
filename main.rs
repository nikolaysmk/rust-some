extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    /* 1. Check the command line arguments */
    if args().len() < 3 {
        println!("Usage: {} <source> <target>", args().nth(0).unwrap());
        return;
    }

    /* 2. Create the input and output files */
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    /* 3. Create a Gzip encoder with the default compression settings */
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    /* 4. Copy the input file to the output file */
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    /* 5. Print some statistics about the output */
    println!(
        "Source length: {}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Compressed length: {}", output.metadata().unwrap().len());
    println!("Compression took: {:?}", start.elapsed());
}
