use proconio::*;

fn main() {
    input! {n: usize, m: usize, f: [usize; n]}

    let mut nf = f.clone();
    nf.sort();
    nf.dedup();

    if nf.len() == f.len() {
        println!("Yes");
    } else {
        println!("No");
    }

    let mut t = vec![false; m + 1];
    for f in f {
        t[f] = true;
    }

    if t[1..].iter().all(|&t| t) {
        println!("Yes")
    } else {
        println!("No")
    }
}
