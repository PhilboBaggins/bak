extern crate clap;
extern crate fs_extra;

use std::path::Path;
use clap::{App, Arg};
//use fs_extra::file::copy_with_progress;
use fs_extra::file::*;

pub fn bak_bak_bak(name: &str, description : &str, gen_name: fn(&str) -> std::string::String) {
    let matches = App::new(name)
        .version("1.0")
        .about(description)
        .author("Phil B.")
        .arg(Arg::with_name("file")
            .help("File to backup")
            .takes_value(true)
            .required(true)
            .multiple(true))
        .get_matches();

    for src_file in matches.values_of("file").unwrap() {
        let new_name = gen_name(src_file);

        let path_from = Path::new(src_file);
        let path_to = Path::new(&new_name);

        let options = CopyOptions::new();
        let handle = |process_info: TransitProcess|
            eprint!("\r{:.2}%", process_info.copied_bytes as f64 / process_info.total_bytes as f64 * 100.0);
        match fs_extra::file::copy_with_progress(path_from, path_to, &options, handle) {
            Ok(_) => eprintln!("\rCopied {} -> {}", path_from.to_str().unwrap(), path_to.to_str().unwrap()),
            Err(x) => eprintln!("\n{}", x),
        }
    }
}
