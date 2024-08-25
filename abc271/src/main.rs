use rand::{self, Rng};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let mut file = std::fs::File::create("testcase.txt").unwrap();

    const N: usize = 20;
    writeln!(file, "{}", N).unwrap();

    for _ in 0..N {
        for j in 0..N {
            if j > 0 {
                write!(file, " ").unwrap();
            }
            let r: i32 = rng.gen_range(0, 1 << 30);
            write!(file, "{}", r).unwrap();
        }
        writeln!(file).unwrap();
    }
}
