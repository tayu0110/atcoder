use proconio::input;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize, usize); n]}

    let (mut l, mut r) = (0, std::usize::MAX >> 2);
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut sum = 0;
        for &(a, b, c) in &p {
            if m < b {
                continue;
            }

            sum += std::cmp::min(1 + (m - b) / c, a);
        }

        if sum >= k {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
