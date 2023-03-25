use rand::{self, Rng};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let (n, m, q): (usize, usize, usize) = (rng.gen_range(1, 10), rng.gen_range(1, 10), rng.gen_range(1, 10));

    let mut file = std::fs::File::create("testcase.txt").unwrap();
    writeln!(file, "{} {} {}", n, m, q).unwrap();

    for _ in 0..q {
        let (a, c): (usize, usize) = 
            (rng.gen_range(1, n+1), rng.gen_range(1, m+1));
        let (b, d): (usize, usize) =
            (rng.gen_range(a, n+1), rng.gen_range(c, m+1));
        writeln!(file, "{} {} {} {}", a, b, c, d).unwrap();
    }
}
