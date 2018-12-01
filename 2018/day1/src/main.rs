use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;

fn main() {
    let mut changes = Vec::new();
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let lines = file.lines();

    for (_, line) in lines.enumerate() {
        let l = line.unwrap();
        if l == "0" || l == "" {
            continue;
        }
        let freq_change = l.parse::<i32>().unwrap();
        changes.push(freq_change);
    }     

    // Initialize frequency at zero per the instructions
    let mut freq = 0;
    let mut freqs_seen = Vec::new();
    freqs_seen.push(freq);

    'outer: loop {
        for change in &changes {
            freq = freq + change;

            let r = freqs_seen.binary_search(&freq);
            match r {
                Ok(_) => {
                    println!("Repeat frequency found: {}", freq);
                    break 'outer;
                },
                Err(pos) => {
                    freqs_seen.insert(pos, freq);
                }
            }
        }
    }
}
