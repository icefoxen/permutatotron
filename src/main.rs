
const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const MAX_LENGTH: usize = 6;

fn print_bytes(v: &[u8]) {
    use std::io::{self, Write};
    let s = v.iter().map(|c| ALPHABET[*c as usize]);
    let mut stdout = io::stdout();
    for c in s {
        let _ = stdout.write(&[c]);
    }
}

fn main() {
    let alphabet_size = ALPHABET.len();
    let mut v = vec![0; MAX_LENGTH];
    
    loop {
        let mut place = 0;
        print_bytes(&v);
        v[place] += 1;
        while v[place] as usize >= alphabet_size {
            v[place] = 0;
            place += 1;
            //println!("V: {:?}", v);
            if place >= MAX_LENGTH {
                println!("Done!");
                return;
            }
            v[place] += 1;
        }
    }    
}
