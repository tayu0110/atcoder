use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut r: [usize; n]}
    r.sort_unstable();
    r.insert(0, 0);
    for i in 0..n {
        r[i + 1] += r[i];
    }

    for _ in 0..q {
        input! {x: usize}

        let pos = r.partition_point(|&r| r <= x);
        println!("{}", pos - 1);
    }
}
