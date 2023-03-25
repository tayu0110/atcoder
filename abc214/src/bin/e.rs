use proconio::input;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut p: [(usize, usize); n]}

        p.sort_by_key(|&(a, b)| (b, a));
    }
}
