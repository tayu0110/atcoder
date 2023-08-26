use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut res = 0;
    let mut min = std::usize::MAX;
    for p in p {
        if p <= min {
            res += 1;
            min = p;
        }
    }

    println!("{}", res)
}
