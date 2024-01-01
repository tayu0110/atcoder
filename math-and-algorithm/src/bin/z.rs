use proconio::*;

const N: [f64; 1000001] = {
    let mut n = [0.; 1000001];
    let mut i = 1;
    let mut sum = 0.;
    while i <= 1000000 {
        sum += 1.0 / i as f64;
        n[i] = sum * i as f64;
        i += 1;
    }
    n
};

fn main() {
    input! {n: u32}
    println!("{}", N[n as usize]);
}
