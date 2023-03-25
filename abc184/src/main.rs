use rand::{self, Rng};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let (n, t) = (rng.gen_range::<usize, usize, usize>(1, 41), rng.gen_range::<usize, usize, usize>(1, 1000000001));
    let mut file = std::fs::File::create("testcase.txt").unwrap();

    writeln!(file, "{} {}", n, t).unwrap();

    for _ in 0..n {
        writeln!(file, "{}", rng.gen_range::<usize, usize, usize>(1, 1000000001)).unwrap();
    }
}
