use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ret = 0;
    for l in 0..n {
        for r in l + 1..=n {
            let sum = a[l..r].iter().sum::<usize>();
            if a[l..r].iter().all(|&a| sum % a != 0) {
                ret += 1;
            }
        }
    }

    println!("{ret}")
}
