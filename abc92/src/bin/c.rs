use proconio::*;

fn main() {
    input! {mut a: [i32]}
    a.insert(0, 0);
    a.push(0);

    let mut res = 0;
    for v in a.windows(2) {
        res += (v[1] - v[0]).abs();
    }

    for v in a.windows(3) {
        println!(
            "{}",
            res - (v[1] - v[0]).abs() - (v[2] - v[1]).abs() + (v[2] - v[0]).abs()
        )
    }
}
