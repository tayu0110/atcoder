use proconio::*;

fn main() {
    input! {n: usize, a: [[usize; 3]; n]}

    for i in 0..n {
        let mut res = 0;
        for j in 0..3 {
            if a.iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .all(|(_, v)| v[j] != a[i][j])
            {
                res += a[i][j];
            }
        }

        println!("{res}")
    }
}
