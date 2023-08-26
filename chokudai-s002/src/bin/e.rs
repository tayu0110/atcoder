use proconio::input;

fn main() {
    input! {n: usize}

    let mut res = 0;
    for _ in 0..n {
        input! {a: usize, b: usize}
        res += b.min(a / 2);
    }

    println!("{}", res)
}
