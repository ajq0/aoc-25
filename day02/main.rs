use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut count: i64 = 0;
    for line in input.trim_end().split(',') {
        let parts: Vec<&str> = line.split('-').collect();
        let mut first = parts[0].to_string();
        if first.len() %2 != 0 {
            first = format!("0{}", first);
        }
        let min:i64 = first.parse()?;
        let max: i64 = parts[1].parse()?;
        let part = &first[0..first.len()/2];
        
        let mut num: i64 = part.parse()?;
        loop {
            let mut res = num.to_string();
            res = format!("{}{}", res, res);
            let res_num: i64 = res.parse()?;
            if res_num > max {
                break;
            }
            if res_num >= min {
                count += res_num;
                println!("{}", res_num)
            }
            num += 1
        }
        
    }

    println!("{}", count);

    Ok(())
}
