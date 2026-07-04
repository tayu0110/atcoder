use proconio::*;

fn main() {
    input! {r: usize}

    let mut res = 2 * r - 1;
    for i in 1..r {
        let (mut lo, mut hi) = (0, r + 1);
        while hi - lo > 1 {
            let j = (hi + lo) / 2;
            if i * i + i + j * j + j < r * r {
                lo = j;
            } else {
                hi = j;
            }
        }
        let j = lo;
        res += 4 * j;
        if i * i + i < r * r {
            res += 2;
        }
    }
    println!("{res}")
}
