use proconio::*;

fn main() {
    input! {s: [marker::Chars; 9]}

    for i in 0..9 {
        let mut s = s[i].clone();
        s.sort();
        s.dedup();
        if s.len() != 9 {
            println!("No");
            return;
        }
    }
    for i in 0..9 {
        let mut t = vec![];
        for j in 0..9 {
            t.push(s[j][i]);
        }
        t.sort();
        t.dedup();

        if t.len() != 9 {
            println!("No");
            return;
        }
    }

    for i in 0..9 {
        for j in 0..9 {
            for (dx, dy) in [(1usize, 2usize), (2, 1)] {
                for (dx, dy) in [(dx, dy),
                    (dx.wrapping_neg(), dy),
                    (dx, dy.wrapping_neg()),
                    (dx.wrapping_neg(), dy.wrapping_neg())] {
                    let (x, y) = (dx.wrapping_add(i), dy.wrapping_add(j));
                    if x < 9 && y < 9 && s[i][j] == s[x][y] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes")
}
