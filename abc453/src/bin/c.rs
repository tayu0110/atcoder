use proconio::*;

fn solve(bits: u32, l: &[i64]) -> u32 {
    let mut now = 0;
    let mut ret = 0;
    for (i, &l) in l.iter().enumerate() {
        if bits & (1 << i) != 0 {
            let next = now + l;
            if now < 0 && next >= 0 {
                ret += 1;
            }
            now = next;
        } else {
            let next = now - l;
            if now >= 0 && next < 0 {
                ret += 1;
            }
            now = next;
        }
    }
    ret
}

fn main() {
    input! {n: usize, l: [i64; n]}

    let mut ret = 0;
    for i in 0..1 << n {
        ret = ret.max(solve(i, &l));
    }
    println!("{ret}");
}
