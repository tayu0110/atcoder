use proconio::*;

fn main() {
    input! {_: usize, s: usize, mut t: usize}

    if t < s {
        println!("-1");
        return;
    }

    for i in 0..30 {
        if s == t {
            println!("{}", i);
            return;
        }

        t >>= 1;
    }

    println!("-1")
}
