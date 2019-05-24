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
        let bf = fs::read_to_string(fname).expect("Error: cannot read file");

        let mut output = String::new();
        let mut p: usize = 0;
        let mut v: Vec<u8> = vec![0];
        let mut l: Vec<usize> = Vec::0;
        let mut i = 0;
        let mut c = ' ';
        while i < bf.chars().len() {
            c = bf.chars()[i]
            println!("Executing: {}", c);
            match c {
                '>' => p += 1,
                '<' => p -= 1,
                '+' => v[p] += 1,
                '-' => v[p] -= 1,
                '[' => {
                        l.push(i);
                    },
                ']' => i = l[l.len() - 1],
                _ => (),
            }
            if p >= v.len() {
                v.push(0);
            }
            dump(&v);
            i += 1;
        }
        println!("Final output: {}", output);
    } else {
        println!("Usage: bfrs filename.bf")
    }
}