use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day04/input")?;
    let mut sum = 0;
    let mut rows: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();
    loop {
        let mut count = 0;
        for y in 0..rows.len() {
            for x in 0..rows[y].len() {
                if let '.' = rows[y][x] {
                    continue;
                }
                let mut free = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        if (y as isize + i as isize - 1) < 0
                            || (y as isize + i as isize - 1) >= rows.len() as isize
                            || x as isize + j as isize - 1 < 0
                            || x as isize + j as isize - 1 >= rows[0].len() as isize
                        {
                            free += 1
                        } else if let '.' = rows[y + i - 1][x + j - 1] {
                            free += 1;
                        }
                    }
                }
                if free > 4 {
                    count += 1;
                    rows[y][x] = '.';
                }
            }
        }
        println!("{}", count);
        sum += count;
        if count == 0 {
            break;
        }
    }

    println!("{}", sum);

    Ok(())
}
