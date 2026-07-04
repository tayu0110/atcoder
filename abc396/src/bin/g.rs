use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [marker::Bytes; h]}

    let a = a
        .into_iter()
        .map(|a| a.into_iter().fold(0, |s, v| (s << 1) | (v - b'0') as u32))
        .collect::<Vec<u32>>();
    let min = (0u32..1 << w)
        .map(|b| {
            let pc = b.count_ones();
            pc.min(w as u32 - pc)
        })
        .collect::<Vec<u32>>();

    let mut res = u32::MAX;
    for mask in 0..1 << w {
        res = res.min(a.iter().map(|&a| min[(a ^ mask) as usize]).sum());
    }
    println!("{res}")
}
