use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Guard {
    minutes: HashMap<i32, i32>,
    total_minutes: i32,
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(&f).lines();

    let mut data: Vec<String> = Vec::new();
    let mut guards: HashMap<String, Guard> = HashMap::new();

    for line in lines {
        let l = line.unwrap();
        if l == "" {
            continue;
        }
        data.push(l);
    }

    data.sort();

    let mut guard_id = String::new();
    let mut last_minute = String::new();
    let mut last_entry = String::new();

    for line in data {
        if line == "" {
            continue;
        }
        let mut index = line.find("]").unwrap();
        let mut timestamp = line.clone();
        let mut entry = timestamp.split_off(index+1);
        entry = entry.trim().to_string();
        let brackets: &[_] = &['[', ']'];
        timestamp = timestamp.trim_matches(brackets).to_string();
        index = timestamp.rfind(":").unwrap();
        let minute = timestamp.split_off(index+1);
        println!("Last Minute: {}, Minute: {}, entry: {}", last_minute, minute, entry);

        if entry.starts_with("Guard") {
            let words = entry.split_whitespace();
            for word in words {
                if word.starts_with("#") {
                    guard_id = word.trim_start_matches('#').to_string();
                }
            }
        } else if entry == "wakes up" {
            // Now we have the guard id, the time they fell asleep in last_minute (always)
            // in the midnight hour), and the time they woke up, lets add these minutes
            let g = guards.entry(guard_id.clone()).or_insert(Guard{minutes: HashMap::new(), total_minutes: 0});

            let start: i32 = last_minute.parse().unwrap();
            let end: i32 = minute.parse().unwrap();
            g.total_minutes += end - start;
            for i in start..end {
                let m = g.minutes.entry(i).or_insert(0);
                *m += 1;
            }
        }

        last_minute = minute;
        last_entry = entry;
    }

    let mut sleepiest_guard = String::new();
    let mut minutes_slept: i32 = 0;
    for (id, guard) in &guards {
        if guard.total_minutes > minutes_slept {
            sleepiest_guard = id.to_string();
            minutes_slept = guard.total_minutes;
        }
    }

    let g = guards.get(&sleepiest_guard).unwrap();
    let mut most_common: i32 = 0;
    let mut most_times: i32 = 0;

    for (minute, times) in &g.minutes {
        if times > &most_times {
            most_common = *minute;
            most_times = *times;
        }
    }

    println!("Guard {} slept {} minutes", sleepiest_guard, minutes_slept);
    println!("Guard {} is asleep at {} the most", sleepiest_guard, most_common);
}
