use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;

    let mut lines = String::new();
    input_file.read_to_string(&mut lines)?;

    let mut arrow: i64 = 50;
    let mut count = 0;
    for line in lines.split_terminator('\n') {
        let mut num: i64 = line[1..].parse()?;
        if let Some('L') = line.chars().next() {
            num *= -1
        }

        let mut next = arrow + num;
        if next <= 0 {
            next *= -1;
            next += 100
        }
        if (next / 100) > 0 {
            count += next / 100;
            if arrow == 0 {
                count -= 1;
            }
        }
        // println!("{} + {}: {}, {}", arrow, line, next, count);

        arrow = (arrow + num) % 100;
        if arrow < 0 {
            arrow += 100
        }
        // if arrow == 0 {
        //     count += 1;
        // }
    }
    println!("{}", count);

    Ok(())
}
