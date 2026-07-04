use proconio::*;

fn main() {
    input! {n: usize, h: [u16; n]}

    let mut res = 1;
    for step in 1..n {
        for start in (0..step).take_while(|&start| start + step < n) {
            let mut rle = None;
            for &h in h[start..].iter().step_by(step) {
                match &mut rle {
                    Some((p, cnt)) if *p == h => *cnt += 1,
                    Some((_, cnt)) => {
                        res = res.max(*cnt);
                        rle = Some((h, 1));
                    }
                    None => rle = Some((h, 1)),
                }
            }

            if let Some((_, cnt)) = rle {
                res = res.max(cnt);
            }
        }
    }
    println!("{res}")
}
