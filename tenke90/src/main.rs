use rand::{Rng, thread_rng};
use std::io::Write;

const N: usize = 20;
const P: usize = 10000;
const A: i32 = 100001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let n = rng.gen::<usize>() % N + 2;
    let p = rng.gen::<usize>() % P + 1;
    let k = rng.gen::<usize>() % (n * (n-1) / 2 + 1);

    let mut file = std::fs::File::create("../in.txt")?;
    writeln!(file, "{} {} {}", n, p, k)?;

    let mut buf: Vec<Vec<i32>> = vec![];

    for i in 0..n {
        let mut tmp = vec![];
        for j in 0..n {
            let k = if j < i {
                buf[j][i]
            } else if i == j {
                0
            } else {
                let mut k = rng.gen::<i32>().abs() % A;
                if k % 131 == 0 || k % 11 == 0 || k % 41 == 0 || k % 23 == 0 || k % 91 == 0 {
                    k = -1;
                }
                k
            };
            tmp.push(k);
            if j == 0 {
                write!(file, "{}", k)?;
            } else if j == n-1 {
                writeln!(file, " {}", k)?;
            } else {
                write!(file, " {}", k)?;
            }
        }
        buf.push(tmp);
    }

    Ok(())
}
