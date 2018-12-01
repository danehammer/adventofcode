use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let lines = file.lines().enumerate();

    // Initialize frequency at zero per the instructions
    let mut freq = 0;

    for (_num, line) in lines {
        let l = line.unwrap();
        if l == "0" || l == "" {
            continue;
        }
        let freq_change = l.parse::<i32>().unwrap();
        
        freq = freq + freq_change;
    }
    
    println!("Final freq: {}", freq);
}
