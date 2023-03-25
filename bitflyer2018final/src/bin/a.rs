use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut res = std::usize::MAX;
    for mut p in p {
        let mut t = 0;
        while p % 10 == 0 {
            t += 1;
            p /= 10;
        }
        res = res.min(t);
    }

    println!("{}", res)
}
