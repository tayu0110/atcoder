use proconio::*;

fn main() {
    input! {x: usize, n: usize, p: [usize; n]}

    if n == 0 {
        println!("{}", x);
        return;
    }

    let mut min = 10000000000;
    let mut res = 0;
    for i in 0..110 {
        if p.contains(&i) {
            continue;
        }

        if x.max(i) - x.min(i) < min {
            res = i;
            min = x.max(i) - x.min(i);
        }
    }

    println!("{}", res)
}
