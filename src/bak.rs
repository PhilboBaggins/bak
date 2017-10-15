extern crate bak_common;
use bak_common::bak_bak_bak;

fn main() {
    bak_bak_bak("bak", |src_file| {
        format!("{}.bak", src_file)
    });
}
