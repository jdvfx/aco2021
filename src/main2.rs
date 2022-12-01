use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut increase_times = 0;
    let mut v: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur: i32 = ip.parse().unwrap_or(0);
                v.push(cur);
            }
        }
    }
    let mut pre: i32 = 0;
    let mut win = v.windows(3);
    for _i in 0..=v.len() {
        let cur: i32 = match win.next() {
            Some(w) => w.iter().sum(),
            None => 0,
        };
        if pre > 0 && cur > pre {
            increase_times += 1;
        }
        pre = cur;
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
