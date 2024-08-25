use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut t = [0usize; 5];
    a.into_iter().for_each(|a| t[a / 100] += 1);
    println!("{}", t[4] * t[1] + t[3] * t[2])
}
