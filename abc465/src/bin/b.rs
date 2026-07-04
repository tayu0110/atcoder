use proconio::*;

fn main() {
    input! {x: usize, y: usize, l: usize, r: usize, a: usize, b: usize}

    let mut ret = 0;
    for i in a..b {
        if (l..r).contains(&i) {
            ret += x;
        } else {
            ret += y;
        }
    }
    println!("{ret}")
}
