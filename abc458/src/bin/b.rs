use proconio::*;

fn main() {
    input! {h: usize, w: usize}
    for i in 0..h {
        for j in 0..w {
            let mut ret = 0;
            for (di, dj) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < h && nj < w {
                    ret += 1;
                }
            }

            if j != 0 {
                print!(" ")
            }
            print!("{ret}")
        }
        println!()
    }
}
