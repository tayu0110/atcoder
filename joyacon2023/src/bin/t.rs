use proconio::input;

fn main() {
    input! {s: usize}

    let mut res = s;
    let s = 100 * (s % 10) + s / 10;
    res += s;
    let s = 100 * (s % 10) + s / 10;
    res += s;
    println!("{}", res);
}
