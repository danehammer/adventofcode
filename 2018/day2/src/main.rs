use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::result::Result;

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(&f).lines();

    let mut ids: Vec<String> = Vec::new();

    for line in lines {
        let l = line.unwrap();
        ids.push(l);
    }

    for id in &ids {
        let ids = ids.clone();
        let r = find_closest(&id, &ids);
        match r {
            Ok(v) => {
                println!("id {} is almost {}", id, v);
                break;
            },
            _ => ()
        }
    }
}

fn find_closest(comp: &String, ids: &Vec<String>) -> Result<String, &'static str> {
    for id in ids {
        if comp == id {
            continue;
        }
        let r = compare_ids(&comp, &id);
        if r.is_ok() {
            return Ok(id.to_string());
        }
    }
    return Err("no closeys found");
}

fn compare_ids(a: &String, b: &String) -> Result<(), &'static str> {
    let pairs = a.chars().zip(b.chars());
    let diffs = pairs.filter(|(aChar, bChar)| aChar != bChar).collect::<Vec<(char, char)>>();
    if diffs.len() == 1 {
        return Ok(());
    }
    return Err("not the closeys");
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
