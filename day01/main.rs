use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;

    let mut lines = String::new();
    input_file.read_to_string(&mut lines)?;

    let mut arrow: i32 = 50;
    let mut count = 0;
    for line in lines.split_terminator('\n') {
        let mut num = line[1..].parse::<i32>()?;
        if let Some('L') = line.chars().next() {
            num *= -1i32
        }

        println!("{}", (num - arrow) / 100);
        arrow = (arrow + num) % 100;
        if arrow < 0 {
            arrow += 100
        }
        println!("{}", arrow);
        if arrow == 0 {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}
