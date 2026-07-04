use proconio::*;

fn main() {
    input! {n: usize, mut e: [(usize, usize); n], q: usize, t: [usize; q]}

    let mut max = 0;
    for (h, _) in e.iter_mut().rev() {
        *h = (*h).max(max);
        max = max.max(*h);
    }

    for t in t {
        let pos = e.partition_point(|e| e.1 <= t);
        println!("{}", e[pos].0);
    }
}
