use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n], x: usize}
    if x < h[0] {
        println!("1");
        return;
    }

    for (i, v) in h.windows(2).enumerate() {
        if v[0] < x && x < v[1] {
            println!("{}", i + 2);
            return;
        }
    }

    println!("{}", n + 1);
}
