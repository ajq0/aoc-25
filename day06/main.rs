use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day06/input")?;

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    let mut result = 0;
    let mut op = '+';
    let l = 5;
    for i in 0..lines.get(0).unwrap().len() {
        let mut num = 0;
        let mut count = 0;
        for x in 0..l {
            let c = lines.get(x).unwrap().get(i..i + 1).unwrap();
            if c == "+" {
                op = '+'
            } else if c == "*" {
                op = '*'
            } else if c != " " {
                let n: u64 = c.parse().unwrap();
                num = num * 10 + n;
            } else {
                count += 1
            }
        }
        if count < l {
            if op == '+' {
                result += num;
            } else {
                if result == 0 {
                    result = num;
                } else {
                    result *= num;
                }
            }
            println!("{} {} {}", num, op, result);
        } else {
            println!("{}", result);
            sum += result;
            result = 0;
        }
    }
    sum += result;
    println!("{}", sum);
    Ok(())
}
