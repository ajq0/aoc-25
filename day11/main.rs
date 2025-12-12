use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day11/input")?;

    let mut dir: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut count: HashMap<&str, u128> = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        dir.insert(from, to.split(' ').collect());
        count.insert(from, 0);
    }

    let from = "dac";
    let to = "out";
    let mut row = vec![from];
    count.insert(from, 1273218246228);
    loop {
        let current = row.clone();
        row.clear();
        for curr in current {
            if curr == "out"{
                continue;
            }
            for next in dir.get(curr).unwrap() {
                count.insert(next, count.get(next).unwrap_or(&0) + count.get(curr).unwrap());
                if *next != to && !row.contains(next) {
                    row.push(next);
                }
            }
        }

        if row.len() == 0 {
            break;
        }
    }
    // println!("The entire map: {:?}", count);
    println!("{} - {}: {}", from, to, count.get(to).unwrap());

    Ok(())
}
