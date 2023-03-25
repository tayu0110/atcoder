use std::io::Write;
use rand::{self, Rng};

fn main() {
    const K_MAX: i64 = 100;
    const XY_MIN: i64 = -100_000;
    const XY_MAX: i64 = 100_000;
    let mut rng = rand::thread_rng();
    let mut file = std::fs::File::create("input.txt").unwrap();

    let k: i64 = rng.gen_range(1, K_MAX);
    let mut x: i64 = rng.gen_range(XY_MIN, XY_MAX);
    let y: i64 = rng.gen_range(XY_MIN, XY_MAX);

    if (x - y).abs() % 2 != k % 2 {
        if x != -100_000 {
            x -= 1;
        } else {
            x += 1;
        }
    }

    writeln!(file, "{}", k).unwrap();
    writeln!(file, "{} {}", x, y).unwrap();
}