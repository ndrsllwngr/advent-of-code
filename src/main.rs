use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut prev_measurement: Option<i64> = None;
        let mut count_increased = 0;
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                let depth: i64 = ip.parse().unwrap();
                if let Some(prev_depth) = prev_measurement {
                    if depth > prev_depth {
                        count_increased = count_increased + 1;
                    }
                }
                prev_measurement = Some(depth);
            }
        }
        println!("count_increased={}", count_increased);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}