use fst::SetBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn main() {
    let input_file = Path::new("fst_builder/data/purls.txt");
    let output_file = Path::new("purls.fst");

    let file = File::create(output_file).expect("Cannot create FST file");
    let mut builder = SetBuilder::new(file).expect("Failed to create FST builder");

    let f = File::open(input_file).expect("Cannot open input file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let s = line.expect("Failed to read line");
        if !s.is_empty() {
            builder.insert(&s).unwrap();
        }
    }

    builder.finish().expect("Failed to finish FST");
    println!("FST generated at {:?}", output_file);
}
