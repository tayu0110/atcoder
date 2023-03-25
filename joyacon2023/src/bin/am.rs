use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, a: usize, b: usize}

    let white = vec!['.'; b];
    let black = vec!['#'; b];

    let mut l1 = vec![];
    let mut l2 = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            l1.append(&mut white.clone());
            l2.append(&mut black.clone());
        } else {
            l2.append(&mut white.clone());
            l1.append(&mut black.clone());
        }
    }

    for i in 0..n {
        if i % 2 == 0 {
            for _ in 0..a {
                println!("{}", l1.iter().join(""))
            }
        } else {
            for _ in 0..a {
                println!("{}", l2.iter().join(""))
            }
        }
    }
}
