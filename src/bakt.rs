extern crate bak_common;
use bak_common::bak_bak_bak;

extern crate chrono;
use chrono::prelude::*;

fn main() {
    bak_bak_bak("bakt", |src_file| {
        let dt: DateTime<Local> = Local::now();
        format!("{}_{}.bak", src_file, dt.format("%Y-%m-%d_%H-%M-%S"))
    });
}
