use rand::{self, Rng};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let mut file = std::fs::File::create("input.txt").unwrap();

    const N: usize = 200000;
    writeln!(file, "{}", N).unwrap();

    for _ in 0..N {
        let len: usize = rng.gen_range(3, 9);
        let mut s = String::new();
        for _ in 0..len {
            let mut c = '\0';
            while c < 'A' || ('Z' < c && c < 'a') || 'z' < c {
                c = rng.gen_range(b'A', b'z'+1) as char;
            }

            s.push(c);
        }

        writeln!(file, "{}", s).unwrap();
    }
}
