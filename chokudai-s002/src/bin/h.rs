use proconio::input;

fn main() {
    input! {n: usize}

    for _ in 0..n {
        input! {a: u32, b: u32}

        if a == b {
            println!("-1")
        } else {
            println!("{}", a.max(b) - a.min(b))
        }
    }
}
