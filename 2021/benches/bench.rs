use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc::*;

macro_rules! bench {
    ($c:expr, $path:path, $day_num:expr) => {{
        use $path::*;
        let s = read_input($day_num);
        $c.bench_function(concat!(stringify!($path), "::part1"), |b| {
            b.iter(|| black_box(part1(black_box(&s))))
        });
        $c.bench_function(concat!(stringify!($path), "::part2"), |b| {
            b.iter(|| black_box(part2(black_box(&s))))
        });
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // bench!(c, day01, 1);
    // bench!(c, day02, 2);
    // bench!(c, day03, 3);
    // bench!(c, day04, 4);
    // bench!(c, day05, 5);
    // bench!(c, day06, 6);
    // bench!(c, day07, 7);
    //bench!(c, day08, 8);
    //bench!(c, day09, 9);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
