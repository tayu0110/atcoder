use rand::{self, Rng};
use std::io::Write;

fn main() {
    const N: usize = 10;

    let mut file = std::fs::File::create("input.txt").unwrap();

    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(5, 10);
    let y: usize = rng.gen_range(1, x+1);

    writeln!(file, "{} {} {}", N, x, y).unwrap();
    for _ in 0..N {
        let o: usize = rng.gen_range(y, x+1);
        writeln!(file, "{}", o).unwrap();
    }
}
