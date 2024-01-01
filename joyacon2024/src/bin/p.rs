use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let mut cnt = 0;
    for a in a {
        if a == cnt + 1 {
            cnt += 1;
        }
    }

    if cnt == 0 {
        println!("-1")
    } else {
        println!("{}", n - cnt);
    }
}
