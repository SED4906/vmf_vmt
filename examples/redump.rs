use std::{env, fs};

use vmf_vmt::{build_vmf_vmt, parse_vmf_vmt};

fn main() {
    for file in env::args().skip(1) {
        let data = fs::read_to_string(file).unwrap();
        println!("{}",build_vmf_vmt(parse_vmf_vmt(&data).unwrap().1));
    }
}