use proconio::*;

fn main() {
    input! {mut n: usize, k: usize}

    let mut cnt = 0;
    for i in 0.. {
        cnt += n;
        if cnt >= k {
            println!("{i}");
            return;
        }
        n += 1;
    }
}
