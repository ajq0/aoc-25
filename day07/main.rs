use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day07/input")?;

    let lines: Vec<&str> = input.lines().collect();
    let mut rays: Vec<u64> = vec![0; lines[0].len()];

    for line in lines.iter() {
        let mut rays2: Vec<u64> = vec![0; line.len()];
        let mut split = false;
        for (index, char) in line.chars().enumerate() {
            let prev = rays[index];
            match char {
                'S' => {
                    rays2[index] = 1;
                    split = true;
                },
                '^' => {
                    if prev > 0 {
                        rays2[index-1] += prev;
                        rays2[index+1] += prev;
                    }
                    split = true;
                }
                _ => {
                    rays2[index] += prev;
                }
            }
        }
        if split {
            rays = rays2;
        }
    }

    println!("{}", rays.iter().fold(0, |a,b| a + b));

    Ok(())
}