use num::integer::Roots;
use proconio::*;

fn main() {
    input! {d: usize}

    let mut res = usize::MAX;
    for x in (0..).take_while(|x| x * x <= d) {
        let rem = d - x * x;
        let y_min = rem.sqrt();
        res = res.min(rem - y_min * y_min);
        res = res.min((y_min + 1) * (y_min + 1) - rem);
    }

    println!("{}", res)
}
