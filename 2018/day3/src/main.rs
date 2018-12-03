use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct Claim {
    id: i16,
    right: i16,
    top: i16,
    width: i16,
    height: i16,
}

fn parse_claim(s: &String) -> Claim {
    let split = s.split(" ");
    let (mut id, mut right, mut top, mut width, mut height) = (0i16, 0i16, 0i16, 0i16, 0i16);
    for part in split {
        if part == "@" {
            continue;
        } else if part.starts_with("#") {
            id = part.trim_matches('#').parse().unwrap();
        } else if part.ends_with(":") {
            let mut dims = part.trim_matches(':').split(",");
            right = dims.next().unwrap().parse().unwrap();
            top = dims.next().unwrap().parse().unwrap();
        } else {
            let mut dims = part.split("x");
            width = dims.next().unwrap().parse().unwrap();
            height = dims.next().unwrap().parse().unwrap();
        }
    }
    return Claim {
        id: id,
        right: right,
        top: top,
        width: width,
        height: height,
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(&f).lines();

    let mut claims: Vec<Claim> = Vec::new();

    for line in lines {
        let l = line.unwrap();
        if l == "" {
            continue;
        }
        claims.push(parse_claim(&l));
    }

    //let mut claim_ids = claims.map(|&x| x.id).collect();
    let mut dims: Vec<Vec<i16>> = Vec::new();
    
    for claim in claims {
        // inner vec is row cells, offset by right
        // outer vec is rows, offset by top

        // make sure we have rows
        for i in 0..(claim.top+claim.height+1) {
            if i > dims.len() as i16 {
                dims.push(Vec::new());
            }
        }
        
        // for each row in the claim
        for i in (claim.top)..(claim.top+claim.height) {
            let mut row = dims[i as usize].clone();

            // make sure we have cells in row
            for j in 0..(claim.right+claim.width+1) {
                if j > row.len() as i16 {
                    row.push(0);
                }
            }

            for j in (claim.right)..(claim.right+claim.width) {
                let cell_claims = row[j as usize] + 1;
                row.remove(j as usize);
                row.insert(j as usize, cell_claims);
            }
            dims.remove(i as usize);
            dims.insert(i as usize, row);
        }
    }

    let mut overlaps = 0;

    for i in 0..dims.len() {
        let row = &dims[i as usize];
        for j in 0..row.len() {
            if row[j] > 1 {
                overlaps += 1;
            }
        }
    }

    //println!("Claim IDs: {:?}", claim_ids);
    println!("Overlaps: {}", overlaps);
}
