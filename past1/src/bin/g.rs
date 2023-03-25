use proconio::input;

fn main() {
    input! {n: usize}
    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {na: [i64; n-i-1]};
        for (a, b) in a[i].iter_mut().skip(i + 1).zip(na.into_iter()) {
            *a = b;
        }
    }

    let mut res = std::i64::MIN;
    for mut i in 0..(3u32.pow(n as u32)) {
        let mut groups = vec![vec![]; 3];
        for j in 0..n {
            groups[i as usize % 3].push(j);
            i /= 3;
        }

        assert_eq!(i, 0);

        let mut score = 0;
        for i in 0..3 {
            for (j, &v) in groups[i].iter().enumerate() {
                for &w in groups[i].iter().skip(j + 1) {
                    score += a[v][w];
                }
            }
        }

        res = std::cmp::max(res, score);
    }

    println!("{}", res);
}
