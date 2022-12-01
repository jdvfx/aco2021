use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut increase_times = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut prev: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let cur: i32 = ip.parse().unwrap_or(0);
                if prev > 0 && cur > prev {
                    increase_times += 1;
                }
                prev = cur;
            }
        }
    }
    println!("> increased times: {}", increase_times);
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
