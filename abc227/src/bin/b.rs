#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: [i32; n]}

    let mut res = 0;
    for v in s {
        res += 1;
        for a in 1..=1000 {
            if v <= 3 * a {
                break;
            }
            if (v - 3 * a) % (4 * a + 3) == 0 {
                res -= 1;
                break;
            }
        }
    }

    println!("{}", res);
}
