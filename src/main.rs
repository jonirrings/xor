use std::io::{Read, Write};
use std::fs::OpenOptions;
use rayon::prelude::*;

fn main() {
    let input = std::env::args().nth(1);
    let output = std::env::args().nth(2);
    match (input, output) {
        (Some(i), Some(o)) => {
            xor(i, o);
        }
        (Some(i), None) => {
            let mut o = i.clone();
            o.push_str(".xml");
            xor(i, o)
        }
        _ => {
            println!("missing parameter!");
            println!("usage:");
            println!("\txor input_file output_file");
        }
    }
}

fn xor(i: String, o: String) {
    let input_file = OpenOptions::new().read(true).open(i);
    if let Err(err) = input_file {
        println!("{}", err);
        return;
    }
    let output_file = OpenOptions::new().write(true).create(true).open(o);
    if let Err(err) = output_file {
        println!("{}", err);
        return;
    }
    if let (Ok(mut input), Ok(mut output)) = (input_file, output_file) {
        let mut buf = Vec::new();
        if input.read_to_end(&mut buf).is_err() {
            println!("Reading Error!");
        };
        buf.par_iter_mut().for_each(|p| *p ^= 0xff);
        if output.write(&buf).is_err() {
            println!("Writing Error!");
        };
    }
}
