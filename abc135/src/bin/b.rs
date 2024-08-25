use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut q = p.clone();
    q.sort_unstable();

    if p.into_iter().zip(q).filter(|(p, q)| p != q).count() <= 2 {
        println!("YES")
    } else {
        println!("NO")
    }
}
