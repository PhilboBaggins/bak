use std::fs;

extern crate chrono;
use chrono::prelude::*;

extern crate bak_common;
use bak_common::bak_bak_bak;

fn main() {
    let name = "baktm";
    let description = "Backup specified file to a new file with the original file' modification date/time followed by \".bak\" at the end of the new file name";
    bak_bak_bak(name, description, |src_file| {
        let file_metadata = fs::metadata(src_file).unwrap();
        if let Ok(file_modification_time) = file_metadata.modified() {
            let dt: DateTime<Local> = file_modification_time.into();
            format!("{}_{}.bak", src_file, dt.format("%Y-%m-%d_%H-%M-%S"))
        } else {
            panic!("Determining file modification date is not supported on this platform");

            // TODO: Should I panic!() here? Or should I return an Err Result instead?
            //       Returning an error seems cleaner but that would mean changing the definition
            //       of this callback function. Returning an error instead of panicing would allow
            //       the program to continue and process other files ... but is that useful? I
            //       guess the real question is: "Could a single run of this program be backing up
            //       2 files, where getting the file modification date/time is supported for one
            //       file but not the other? E.g. files on different filesystems perhaps...
        }
    });
}
