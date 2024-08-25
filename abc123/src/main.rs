use rand::{self, Rng};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let (x, y, z): (usize, usize, usize) = (rng.gen_range(1, 10), rng.gen_range(1, 10), rng.gen_range(1, 10));
    // let (x, y, z) = (1, 1, 1);
    let k = rng.gen_range(1, std::cmp::min(3001, x * y * z + 1));
    // let k = 1;

    let mut file = std::fs::File::create("testcase.txt").unwrap();
    writeln!(file, "{} {} {} {}", x, y, z, k).unwrap();

    for i in 0..x {
        if i > 0 {
            write!(file, " ").unwrap();
        }
        write!(file, "{}", rng.gen_range::<usize, usize, usize>(1, 100)).unwrap();
    }
    writeln!(file).unwrap();
    for i in 0..y {
        if i > 0 {
            write!(file, " ").unwrap();
        }
        write!(file, "{}", rng.gen_range::<usize, usize, usize>(1, 100)).unwrap();
    }
    writeln!(file).unwrap();
    for i in 0..z {
        if i > 0 {
            write!(file, " ").unwrap();
        }
        write!(file, "{}", rng.gen_range::<usize, usize, usize>(1, 100)).unwrap();
    }
    writeln!(file).unwrap();
}
