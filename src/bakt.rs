extern crate chrono;
use chrono::prelude::*;

extern crate bak_common;
use bak_common::bak_bak_bak;

fn main() {
    let name = "bakt";
    let description = "Backup specified file to a new file with the current date/time followed by \".bak\" at the end of the new file name";
    bak_bak_bak(name, description, |src_file| {
        let dt: DateTime<Local> = Local::now();
        format!("{}_{}.bak", src_file, dt.format("%Y-%m-%d_%H-%M-%S"))
    });
}
