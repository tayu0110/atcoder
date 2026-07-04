use proconio::*;

fn main() {
    input! {n: usize, mut e: [(usize, usize); n]}
    e.sort_unstable();

    let mut ret = 0;
    let mut px = 0;
    let mut py = usize::MAX;
    for (x, y) in e {
        if px < x && py > y {
            ret += 1;
            px = x;
            py = y;
        }
    }
    println!("{}", ret);
}
