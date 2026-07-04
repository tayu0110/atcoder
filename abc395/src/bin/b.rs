use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            let k = n - 1 - i;
            let l = n - 1 - j;
            if i.min(j).min(k).min(l) % 2 == 0 {
                res[i][j] = '#';
            }
        }
    }

    for res in res {
        println!("{}", res.iter().collect::<String>())
    }
}
