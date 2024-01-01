use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut res = 0;
    for i in 0..n {
        res = res.max(h[i..].windows(2).take_while(|v| v[0] >= v[1]).count());
    }

    println!("{res}")
}
