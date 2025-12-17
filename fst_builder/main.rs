/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

use fst::SetBuilder;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn insert_purls(input_file: &Path, builder: &mut SetBuilder<File>) -> usize {
    let f = File::open(input_file).expect("Cannot open input file");
    let reader = BufReader::new(f);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .filter(|s| !s.is_empty())
        .collect();

    lines.sort();
    let count = lines.len();

    println!("Insert PURLs from {:?} in FST", input_file);
    for line in lines {
        builder.insert(&line).unwrap();
    }

    count
}

fn get_txt_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = read_dir(dir)
        .expect("Cannot read directory")
        .map(|entry| entry.expect("Invalid directory entry").path())
        .filter(|path| path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("txt"))
        .collect();

    files.sort();

    files
}

fn main() {
    let input_dir = Path::new("fst_builder/data");
    let output_file = Path::new("purls.fst");
    let mut total_purls: usize = 0;

    let file = File::create(output_file).expect("Cannot create FST file");
    let mut builder = SetBuilder::new(file).expect("Failed to create FST builder");

    let input_files = get_txt_files(input_dir);

    for input_file in input_files {
        total_purls += insert_purls(&input_file, &mut builder);
    }

    builder.finish().expect("Failed to finish FST");
    println!("FST generated with {} base PackageURLs", total_purls);
    println!("FST generated at {:?}", output_file);
}
