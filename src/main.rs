use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    if let Ok(lines) = read_lines("input/day1.txt") {

        let mut prev_measurement: Option<i64> = None;
        let mut count_increased = 0;
        for line in lines {
            if let Ok(ip) = line {

                let depth: i64 = ip.parse().unwrap();
                if let Some(prev_depth) = prev_measurement {
                    if depth > prev_depth {
                        count_increased = count_increased + 1;
                    }
                }
                prev_measurement = Some(depth);
            }
        }
        println!("task1={}", count_increased);
    }


    if let Ok(lines) = read_lines("input/day1.txt") {

        let mut sliding_window: Vec<i64> = Vec::new();
        let mut count_increased = 0;
        for line in lines {
            if let Ok(ip) = line {

                let depth: i64 = ip.parse().unwrap();
                if sliding_window.len() < 3 {
                    println!("len={}, depth={}", &sliding_window.len(), &depth);
                    sliding_window.push(depth);
                    println!("{:?}", &sliding_window);
                } else {
                    let prev_sum: i64 = sliding_window.iter().sum();
                    sliding_window.remove(0);
                    sliding_window.push(depth);
                    let curr_sum: i64 = sliding_window.iter().sum();
                    println!("{:?}", &sliding_window);

                    if curr_sum > prev_sum {
                        count_increased = count_increased + 1;
                    }
                }
            }
        }
        println!("task2={}", count_increased);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
