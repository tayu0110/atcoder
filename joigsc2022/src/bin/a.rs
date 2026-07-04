use proconio::*;

fn main() {
    input! {n: usize, mut p: [(u32, u32); n]}
    p.sort_unstable_by_key(|p| p.1);
    let mut ans = u32::MAX;
    let mut f = p.pop().map(|p| p.0 + p.1).unwrap();
    let mut s = p.pop().map(|p| p.0 + p.1).unwrap();
    if f > s {
        (f, s) = (s, f);
    }
    for (a, b) in p.into_iter().rev() {
        ans = ans.min(a + f + s);
        let t = a + b;
        if t < f {
            (f, s) = (t, f);
        } else if t < s {
            s = t;
        }
    }
    println!("{}", ans);
}
