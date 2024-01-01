use math::miller_rabin_test;
use proconio::*;

fn main() {
    input! {n: u64}

    if miller_rabin_test(n) {
        println!("YES")
    } else {
        println!("NO")
    }
}
