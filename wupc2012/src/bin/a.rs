use proconio::*;

fn main() {
    input! {ma: usize, da: usize, mb: usize, db: usize}

    let mut cnt = 0;
    let mut start = false;
    for (i, d) in [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        .into_iter()
        .enumerate()
    {
        for j in 0..d {
            if i + 1 == ma && j + 1 == da {
                start = true;
            }
            if i + 1 == mb && j + 1 == db {
                start = false;
            }
            if start {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt)
}
