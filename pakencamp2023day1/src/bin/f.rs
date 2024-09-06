use itertools::Itertools;
use proconio::*;

const RESULT: [u32; 32] = {
    let mut result = [0; 32];
    result[1] = 1000000000;
    let mut i = 2;
    while i < 32 {
        result[i] = result[i - 1] / 2;
        i += 1;
    }
    result
};

fn main() {
    input! {n: usize}
    if n > 31 {
        println!("No")
    } else {
        println!("Yes");
        println!("{}", RESULT[..n].iter().join(" "))
    }
}
