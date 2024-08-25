use proconio::*;

const M: u32 = 1000000007;

fn main() {
    input! {s: marker::Bytes}

    let mut memo = [0; 13];
    memo[0] = 1;
    for c in s {
        let mut new = [0; 13];
        if c == b'?' {
            for i in 0..10 {
                for j in 0..13 {
                    new[(j * 10 + i) % 13] += memo[j];
                    new[(j * 10 + i) % 13] %= M;
                }
            }
        } else {
            let c = (c - b'0') as usize;
            for i in 0..13 {
                new[(i * 10 + c) % 13] += memo[i];
                new[(i * 10 + c) % 13] %= M;
            }
        }

        memo = new;
    }

    println!("{}", memo[5]);
}
