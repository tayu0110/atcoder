use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    for i in 0..h {
        for j in 0..w {
            let mut sum = 0;
            for (dx, dy) in vec![
                (0, 1),
                (1, 0),
                (0, 1usize.wrapping_neg()),
                (1usize.wrapping_neg(), 0),
            ] {
                let ni = i.wrapping_add(dy);
                let nj = j.wrapping_add(dx);

                if ni >= h || nj >= w {
                    continue;
                }
                if s[ni][nj] == '#' {
                    sum += 1;
                }
            }

            if s[i][j] == '.' && sum >= 2 {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
