use proconio::*;

fn main() {
    input! {s: String, t: String}

    let mut res = 0;
    let mut cnt = 0;
    let mut over = false;
    for y in 2022.. {
        let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
            days[1] += 1;
        }
        for m in 1..=12 {
            for d in 1..=days[m - 1] {
                let u = format!("{y:04}-{m:02}-{d:02}");
                over |= s == u;
                if over && cnt % 7 <= 1 {
                    res += 1;
                }
                cnt += 1;
                if t == u {
                    println!("{res}");
                    return;
                }
            }
        }
    }
}
