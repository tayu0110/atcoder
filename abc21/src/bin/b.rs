use proconio::*;

fn main() {
    input! {_: usize, a: usize, b: usize, k: usize, mut p: [usize; k]}
    p.push(a);
    p.push(b);

    p.sort_unstable();
    p.dedup();

    if p.len() == k + 2 {
        println!("YES")
    } else {
        println!("NO")
    }
}
