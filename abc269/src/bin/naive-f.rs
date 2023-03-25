use proconio::input;

fn main() {
    input! {_n: usize, m: i64, q: i64, p: [(i64, i64, i64, i64); q]}

    for (a, b, c, d) in p {
        let mut res = 0;
        for i in a..=b {
            for j in c..=d {
                if (i + j) % 2 == 1 {
                    continue;
                }
                res += m * (i-1) + j;
            }
        }

        println!("{}", res);
    }
}