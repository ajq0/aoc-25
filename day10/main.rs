use std::error::Error;
use std::fs;
use itertools::Itertools;


fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day10/input")?;

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for line in lines {
        let (lights, rest) = line.split_once(' ').unwrap();

        let mut target = 0;
        for c in lights.chars().rev() {
            match c {
                '#' => {
                    target = (target << 1) | 1
                }
                '.' => {
                    target <<= 1;
                }
                _ => {}
            }
        }
        
        let (ops_str, jolts) = rest.rsplit_once(' ').unwrap();
        let ops: Vec<&str> = ops_str.split(' ').collect();
        let mut ops_nums: Vec<i32> = vec![];

        for op in ops {
            let mut num = 0;
            for c in op.chars(){
                let n: i32 = c.to_digit(10).unwrap_or(99) as i32;
                if n < 99 {
                    num |= 1 << n
                }
            }
            ops_nums.push(num);
        }

        let mut least = 99;
        for set in ops_nums.iter().powerset() {
            let mut result = 0;
            for op in set.iter() {
                result ^= *op;
                // println!("{}", *op);
            }
            if result == target && set.len() < least {
                least = set.len()
            }
        }
        println!("{}", least);

        sum += least
    }

    println!("{}", sum);
    Ok(())
}