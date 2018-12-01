use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;

fn main() {
    // Initialize frequency at zero per the instructions
    let mut freq = 0;
    let mut freqs_seen = Vec::new();
    freqs_seen.push(freq);

    'outer: loop {
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
        let lines = file.lines();

        for (_, line) in lines.enumerate() {
            let l = line.unwrap();
            if l == "0" || l == "" {
                continue;
            }
            let freq_change = l.parse::<i32>().unwrap();
            
            freq = freq + freq_change;

            // Check if we've seen this frequency before
            let r = freqs_seen.binary_search(&freq);
            if r.is_ok() {
                println!("Repeat frequency found: {}", freq);
                break 'outer;
            }

            // Otherwise note that we've seen it
            freqs_seen.push(freq);
            // Re-sort
            freqs_seen.sort_unstable();
        }
        println!("{}", freqs_seen.len());
    }
}
