use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: i32, b: i32, c: i32}
    for i in -1000..=1000 {
        let sum = a + b + i;
        let mut t = [[a, b, i], [0, c, 0], [0, 0, 0]];
        t[2][0] = sum - c - i;
        t[1][0] = sum - a - t[2][0];
        t[1][2] = sum - c - t[1][0];
        t[2][2] = sum - i - t[1][2];
        t[2][1] = sum - b - c;

        if t[2].iter().sum::<i32>() == sum && (0..3).map(|i| t[i][i]).sum::<i32>() == sum {
            t.iter().for_each(|v| println!("{}", v.iter().join(" ")));
            return;
        }
    }

    unreachable!()
}
