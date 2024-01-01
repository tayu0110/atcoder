use proconio::*;

fn main() {
    input! {l: usize, r: usize, l: [usize; l], r: [usize; r]}

    let l = {
        let mut buf = [0; 41];
        for l in l {
            buf[l] += 1;
        }
        buf
    };
    let r = {
        let mut buf = [0; 41];
        for r in r {
            buf[r] += 1;
        }
        buf
    };

    println!(
        "{}",
        l.into_iter().zip(r).map(|(l, r)| l.min(r)).sum::<usize>()
    )
}
