use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], w: [usize; n]}

    let mut b = vec![vec![]; n + 1];
    for (a, w) in a.into_iter().zip(w) {
        b[a].push(w);
    }

    let mut res = 0;
    for mut b in b {
        if b.len() > 1 {
            b.sort_unstable();
            b.pop();
            res += b.into_iter().sum::<usize>();
        }
    }

    println!("{res}")
}
