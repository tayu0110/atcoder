use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize}

    let mut res = vec![];
    for i in 2..=n {
        let mut bad = false;
        for j in (2..i).take_while(|&j| j * j <= i) {
            if i % j == 0 {
                bad = true;
                break;
            }
        }

        if !bad {
            res.push(i);
        }
    }

    println!("{}", res.iter().join(" "))
}
