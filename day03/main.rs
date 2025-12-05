use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day03/input")?;

    let mut res: u64 = 0;
    for line in input.trim_end().split('\n') {
        let mut n: [u32; 12] = [0; 12];
        for (index, char) in line.chars().enumerate() {
            let x = char.to_digit(10).unwrap();
            for j in 0..12 {
                let e = (index as isize) + (11 - (j as isize)) < (line.len() as isize);
                // println!("{} {} {} {}", e, index, j, line.len());
                if x > n[j] && e {
                    n[j] = x;
                    for k in (j + 1)..12 {
                        n[k] = 0;
                    }
                    break;
                }
            }
        }
        // println!("{:?}", n);
        let mut r = 0;
        for i in 0..12 {
            r += n[i] as u64 * (10u64.pow(11 - i as u32));
            // print!("{} ", (10u64.pow(11 - i as u32)));
        }
        res += r;
    }

    println!("{}", res);

    Ok(())
}
