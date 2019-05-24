extern crate regex;

use regex::Regex;
use std::fs;
use std::env;

fn dump(v: &Vec<u8>) {
    for (p, i) in v.iter().enumerate() {
        println!("[{}] = {}", p, i)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(fname) = &args.get(1) {
        let re = Regex::new(r"[^\+\-<>\[\],.]").unwrap();
        let fcont = fs::read_to_string(fname).expect("Error: cannot read file");
        let bf = re.replace_all(&fcont, "");
        println!("Brainf*** code: {}", bf);

        let mut output = String::new();
        let mut p: usize = 0;
        let mut v: Vec<u8> = Vec::new();
        for c in bf.chars() {
            match c {
                '>' => p += 1,
                '<' => p -= 1,
                '+' => v[p] += 1,
                '-' => v[p] -= 1,
                _ => (),
            }
            if p >= v.len() {
                v.push(0);
            }
            dump(&v);
        }
        println!("Final output: {}", output);
    } else {
        println!("Usage: bfrs filename.bf")
    }
}