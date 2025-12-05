use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day01/input")?;

    let mut lines = String::new();
    input_file.read_to_string(&mut lines)?;

    let mut arrow: i64 = 50;
    let mut count = 0;
    for line in lines.split_terminator('\n') {
        let mut num: i64 = line[1..].parse()?;
        if let Some('L') = line.chars().next() {
            num *= -1
        }

        // 50 -150 = -100 -> 2
        // 50 - 149 = -99 -> 1
        // 50 - 50 = 0 -> 1
        // 50 - 49 = 1 -> 0
        // 50 + 49 = 99 -> 0
        // 50 + 50 = 100 -> 1
        let mut d = arrow + num;
        if d <= 0 && arrow > 0 {
            d -= 100;
        }
        let e = (d / 100).abs();
        count += e;

        arrow = (arrow + num) % 100;
        if arrow < 0 {
            arrow += 100
        }
    }
    println!("{}", count);

    Ok(())
}
