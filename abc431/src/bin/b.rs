use proconio::*;

fn main() {
    input! {mut x: usize, n: usize, w: [usize; n], q: usize, query: [usize; q]}

    let mut t = vec![false; n];
    for p in query {
        let p = p - 1;
        if t[p] {
            x -= w[p];
        } else {
            x += w[p];
        }
        t[p] = !t[p];
        println!("{}", x)
    }
}
