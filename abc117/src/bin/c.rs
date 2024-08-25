use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut x: [i32; m]}

    if n >= m {
        println!("0");
        return;
    }

    x.sort();

    let mut t = x.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    t.sort();

    println!("{}", t[..t.len() - (n - 1)].iter().sum::<i32>())
}
