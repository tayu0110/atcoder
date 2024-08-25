use proconio::*;

fn transpose(s: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = s.len();
    let m = s[0].len();

    let mut res = vec![vec![0; n]; m];
    for i in 0..n {
        for j in 0..m {
            res[j][i] = s[i][j];
        }
    }
    res
}

fn main() {
    input! {n: usize, _: usize, d: usize, mut s: [marker::Bytes; n]}

    let mut res = 0;
    for _ in 0..2 {
        res += s
            .iter()
            .map(|s| {
                s.windows(d)
                    .filter(|s| s.iter().all(|&c| c == b'.'))
                    .count()
            })
            .sum::<usize>();
        s = transpose(s);
    }

    println!("{res}")
}
