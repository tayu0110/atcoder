use proconio::input;

fn main() {
    input! {n: usize, a: [(usize, usize); n], q: usize, b: [(usize, usize); q]};

    let mut score = vec![vec![0; n+1]; 2];
    for (i, (c, p)) in a.into_iter().enumerate() {
        score[c-1][i+1] += p;
        for j in 0..2 {
            score[j][i+1] += score[j][i];
        }
    }

    for (l, r) in b {
        println!("{} {}", score[0][r] - score[0][l-1], score[1][r] - score[1][l-1]);
    }
}