use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let mut res = vec![vec!['#'; 100]; 100];
    for i in 51..100 {
        for j in 0..100 {
            res[i][j] = '.';
        }
    }

    let (mut a, mut b) = (a - 1, b - 1);

    for i in 0..50 {
        if i % 2 == 1 {
            continue;
        }

        for j in 0..100 {
            if j % 2 == 1 {
                continue;
            }

            if a == 0 {
                break;
            }

            a -= 1;
            res[i][j] = '.';
        }
    }

    for i in 52..100 {
        if i % 2 == 1 {
            continue;
        }

        for j in 0..100 {
            if j % 2 == 1 {
                continue;
            }

            if b == 0 {
                continue;
            }

            b -= 1;
            res[i][j] = '#';
        }
    }

    println!("100 100");
    for r in res {
        println!("{}", r.into_iter().join(""))
    }
}
