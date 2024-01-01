use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut db = vec![[0; 31]; n + 1];
    for i in 1..=n {
        let s = i.to_string().bytes().map(|c| c - b'0').sum::<u8>();
        db[i][0] = i - s as usize;
    }

    for i in 1..31 {
        for j in 1..=n {
            db[j][i] = db[db[j][i - 1]][i - 1];
        }
    }

    for i in 1..=n {
        let mut k = k;
        let mut now = i;

        for j in (0..31).rev() {
            if k >= 1 << j {
                k ^= 1 << j;
                now = db[now][j];
            }
        }

        println!("{now}");
    }
}
