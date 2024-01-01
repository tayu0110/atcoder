use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort_unstable();
    let max = a[n - 1];
    while let Some(na) = a.pop() {
        if na == max {
            continue;
        } else {
            a.push(na);
            break;
        }
    }

    println!("{}", a.last().unwrap())
}
