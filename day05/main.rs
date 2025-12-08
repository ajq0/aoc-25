use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day05/input")?;

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    if let [fresh, stuff] = parts[..] {
        let mut ranges: Vec<(u128, u128)> = fresh
            .lines()
            .map(|line| {
                let mut parts = line.split('-').map(|n| n.parse().unwrap());
                let start = parts.next().unwrap();
                let end = parts.next().unwrap();
                (start, end)
            })
            .collect();

        let mut count = 0;
        for item in stuff.lines() {
            let num: u128 = item.parse()?;
            for (min, max) in &ranges {
                if num >= *min && num <= *max {
                    count += 1;
                    break;
                }
            }
        }
        println!("{}", count);
        let mut extra: Vec<(u128, u128)> = vec![];
        for i in 0..(ranges.len()-1) {
            let (head, tail) = ranges.split_at_mut(i+1);
            let (min, max) = &mut head[i];
            if min > max {
                continue;
            }
            for (x,y) in tail.iter_mut() {
                if min >= y || max <= x {
                    continue;
                }
                if min <= x {
                    if max < y { 
                        *x = *max + 1 
                    } else { 
                        *y = *x - 1 
                    }
                } else {
                    if max > y {
                        *y = *min - 1
                    } else {
                        extra.push((*max+1, *y));
                        *y = *min - 1;
                    }
                }
            }
        }

        let mut sum = 0;
        for (min, max) in &ranges {
            // println!("{} {} {} ",min, max, max + 1 - min);
            sum += max + 1 - min
        }
        for (min, max) in &extra {
            // println!("{} {} {} ",min, max, max + 1 - min);
            sum += max + 1 - min
        }
        println!("{}", sum);
    }

    Ok(())
}
