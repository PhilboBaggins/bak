extern crate clap;
extern crate fs_extra;

use clap::{crate_version, Arg, ArgAction, Command};
use std::path::Path;
//use fs_extra::file::copy_with_progress;
use fs_extra::file::*;

pub fn bak_bak_bak(
    name: &'static str,
    description: &'static str,
    gen_name: fn(&str) -> std::string::String,
) {
    let matches = Command::new(name)
        .version(crate_version!())
        .about(description)
        .author("Phil B.")
        .arg(
            Arg::new("file")
                .help("File to backup")
                .action(ArgAction::Append)
                .required(true),
        )
        .get_matches();

    let files_to_backup = matches.get_many::<String>("file").unwrap();
    for src_file in files_to_backup {
        let new_name = gen_name(&src_file);

        let path_from = Path::new(&src_file);
        let path_to = Path::new(&new_name);

        let options = CopyOptions::new();
        let handle = |process_info: TransitProcess| {
            eprint!(
                "\r{:.2}%",
                process_info.copied_bytes as f64 / process_info.total_bytes as f64 * 100.0
            )
        };
        match fs_extra::file::copy_with_progress(path_from, path_to, &options, handle) {
            Ok(_) => eprintln!(
                "\rCopied {} -> {}",
                path_from.to_str().unwrap(),
                path_to.to_str().unwrap()
            ),
            Err(x) => eprintln!("\n{}", x),
        }
    }
}
