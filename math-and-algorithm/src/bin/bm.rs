use proconio::*;

const RES: [u16; 200001] = {
    let mut res = [0; 200001];
    let mut n = 1000;
    while n < res.len() as u32 {
        let r = {
            let (n, mut r) = (n % 10000, n / 10000);
            let (n, k) = (n % 5000, n / 5000);
            r += k;
            r += n / 1000;
            r
        };
        res[n as usize] = r as u16;
        n += 1;
    }
    res
};

#[fastout]
fn main() {
    input! {n: u32}
    println!("{}", RES[n as usize]);
}
