use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut check = vec![false; m + 1];
    for _ in 0..n {
        input! {l: usize, mut x: [usize; l]}
        x.reverse();

        let mut ret = 0;
        while let Some(x) = x.pop() {
            if !check[x] {
                check[x] = true;
                ret = x;
                break;
            }
        }

        println!("{ret}")
    }
}
