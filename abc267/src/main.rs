use std::io::Write;

use rand::{
    thread_rng, Rng
};

fn main() {
    const N: usize = 10;

    let mut file = std::fs::File::create("./input.txt").unwrap();
    writeln!(file, "{}", N).unwrap();

    let mut rng = thread_rng();

    for i in 2..=N {
        let a = i;
        let b = rng.gen_range(1, i);

        writeln!(file, "{} {}", a, b).unwrap();
    }

    writeln!(file, "{}", N * N).unwrap();
    for i in 1..=N {
        for j in 1..=N {
            writeln!(file, "{} {}", i, j).unwrap();
        }
    }
}
