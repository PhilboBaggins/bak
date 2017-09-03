extern crate chrono;
extern crate clap;
extern crate fs_extra;

use std::path::Path;
use chrono::prelude::*;
use clap::{App, Arg};
//use fs_extra::file::copy_with_progress;
use fs_extra::file::*;

fn main() {
    let matches = App::new("bakt")
        .version("1.0")
        .about("Does great things!")
        .author("Phil B.")
        .arg(Arg::with_name("file")
            .help("File to backup")
            .takes_value(true))
        .get_matches();

    if let Some(src_file) = matches.value_of("file") {
        let dt: DateTime<Local> = Local::now();
   
        let new_name = format!("{}_{}.bak", src_file, dt.format("%Y-%m-%d_%H-%M-%S"));

        let path_from = Path::new(src_file);
        let path_to = Path::new(&new_name);

        let options = CopyOptions::new();
        let handle = |process_info: TransitProcess|
            print!("\r{:.2}%", process_info.copied_bytes as f64 / process_info.total_bytes as f64 * 100.0);
        match fs_extra::file::copy_with_progress(path_from, path_to, &options, handle) {
            Ok(_) => println!("\rCopied {} -> {}", path_from.to_str().unwrap(), path_to.to_str().unwrap()),
            Err(x) => println!("\n{:?}", x),
        }
    }
    else {
        println!("WAT?!");
    }
}
