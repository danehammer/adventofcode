use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut twofers = 0;
    let mut threefers = 0;

    for (_, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if has_exactly(2, &l) {
            twofers += 1;
        }
        if has_exactly(3, &l) {
            threefers += 1;
        }
    }

    println!("Twofers: {}", twofers);
    println!("Threefers: {}", threefers);
    println!("Checksum: {}", twofers * threefers);
}

fn has_exactly(num: i8, s: &String) -> bool {
    let mut char_counts: HashMap<char, i8> = HashMap::new();
    for c in s.chars() {
        let val = char_counts.entry(c).or_insert(0);
        *val += 1;
    }
    for val in char_counts.values() {
        if val == &num {
            return true;
        }
    }
    return false;
}
