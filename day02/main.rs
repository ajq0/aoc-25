use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day02/test")?;

    let mut count: i64 = 0;
    for line in input.trim_end().split(',') {
        let parts: Vec<&str> = line.split('-').collect();
        let first = parts[0].to_string();
        let min: u64 = first.parse()?;
        let last = parts[1].to_string();
        let max: u64 = last.parse()?;

        for l in 1..last.len() + 1 {
            if (l > first.len()) {}

            let seq = &first[0..l];
            let mut res = String::from(seq);

            for mul in l..last.len() + 1 {
                res.push_str(seq);
            }

            let num: u64 = res.parse()?;
            if num >= min && num <= max {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
