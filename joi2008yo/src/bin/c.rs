use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();

    let mut b = vec![];
    for i in 1..=2 * n {
        if !a.contains(&i) {
            b.push(i);
        }
    }
    a.sort();

    let mut now = 0;
    while !a.is_empty() && !b.is_empty() {
        if let Some(p) = a.iter().position(|&a| a > now) {
            now = a.remove(p);
        } else {
            now = 0;
        }

        if a.is_empty() {
            break;
        }

        if let Some(p) = b.iter().position(|&b| b > now) {
            now = b.remove(p);
        } else {
            now = 0;
        }
    }

    println!("{}", b.len());
    println!("{}", a.len());
}
