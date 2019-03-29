use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(&f).lines();

    for line in lines {
        let coords = line.split(" ");
        let x = coords.next();
        let y = coords.next().trim();
        println!("X: {}, Y: {}", x, y);
    }
}
