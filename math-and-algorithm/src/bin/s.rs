use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut t = [0; 3];
    a.into_iter().for_each(|a| t[a - 1] += 1);
    let f = |t: usize| t * t.saturating_sub(1) / 2;
    println!("{}", f(t[0]) + f(t[1]) + f(t[2]))
}
