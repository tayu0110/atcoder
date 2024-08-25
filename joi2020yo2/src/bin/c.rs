use proconio::*;

const D: usize = 10000;
const MAX: usize = 1000001;
const DIGSUM: [usize; D + 1] = {
    let mut buf = [0; D + 1];
    let mut i = 1;
    while i < D + 1 {
        let mut now = i;
        let mut sum = 0;
        while now > 0 {
            sum += now % 10;
            now /= 10;
        }
        buf[i] = sum;
        i += 1;
    }
    buf
};
const RESULT: [u32; MAX + 60] = {
    let mut buf = [1; MAX + 60];
    let mut i = 1;
    while i < MAX {
        let sum = DIGSUM[i % D] + DIGSUM[i / D];
        buf[i + sum] += buf[i];
        i += 1;
    }
    buf
};

fn main() {
    input! {n: usize}
    println!("{}", RESULT[n]);
}
