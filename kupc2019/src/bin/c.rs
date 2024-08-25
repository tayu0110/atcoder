use proconio::*;

fn main() {
    input! {m: usize, k: usize}

    let mut now;
    let mut res = 1;
    let mut max = k;
    while max < m {
        now = max * 2 + 1;
        res += 1;
        max += now * k;
    }
    println!("{res}")
}
