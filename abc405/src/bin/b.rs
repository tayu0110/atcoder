use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}

    let mut cnt = vec![0; m + 1];
    for &a in &a {
        cnt[a] += 1;
    }

    let mut res = 0;
    while cnt.iter().skip(1).all(|&c| c > 0) {
        let a = a.pop().unwrap();
        cnt[a] -= 1;
        res += 1;
    }

    println!("{res}")
}
