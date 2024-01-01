use math::Sieve;
use proconio::*;

const S: Sieve = Sieve::new();

#[fastout]
fn main() {
    input! {n: u32}

    for i in 0..S.count() {
        if S[i] > n as usize {
            break;
        }

        println!("{}", S[i]);
    }
}
