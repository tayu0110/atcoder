use proconio::*;

fn main() {
    input! {n: usize, mut m: usize, p: [(u32, u32, u8); n]}

    let mut base = p
        .iter()
        .map(|&(a, b, x)| a as usize * (b as usize + x as usize - 1))
        .sum::<usize>();
    let mut nt = Vec::with_capacity(n * 2);
    for (a, b, x) in p {
        if x == 0 {
            nt.push((a, b - 1));
        } else if b == 1 {
            nt.push((a, 1));
        } else {
            nt.push((a * 2, 1));
            nt.push((a, b - 2));
        }
    }
    nt.sort_unstable();

    while let Some((a, b)) = nt.pop() {
        if m == 0 {
            break;
        }
        base -= a as usize * m.min(b as usize);
        m = m.saturating_sub(b as usize);
    }

    println!("{base}")
}
