extern crate bak_common;
use bak_common::bak_bak_bak;

fn main() {
    let name = "bak";
    let description = "Backup specified file to a new file with \".bak\" at the end of the new file name";
    bak_bak_bak(name, description, |src_file| {
        format!("{}.bak", src_file)
    });
}
