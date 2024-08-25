use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0usize;
    for i in 0..30 {
        let mut b = vec![];
        for &a in &a {
            b.push((a >> i) & 1);
        }

        let mut cum = b.clone();
        for j in 0..n - 1 {
            cum[j + 1] += cum[j];
        }

        let mut cnt = [0; 2];
        for j in 0..n {
            cnt[cum[j] % 2] += 1;
        }

        let mut bit = 1;
        for j in 0..n {
            cnt[cum[j] % 2] -= 1;
            res += cnt[bit] << i;
            if b[j] == 1 {
                bit = (bit + 1) % 2;
            }
        }
    }

    println!("{res}")
}
