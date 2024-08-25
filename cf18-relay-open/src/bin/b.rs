use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: String, gx: i32, gy: i32}

    if gx == 0 && gy == 0 {
        println!("Yes");
        return;
    }

    for v in b"LRUD".into_iter().cloned().permutations(4) {
        let s = s
            .replace('W', format!("{}", v[0] as char).as_str())
            .replace('X', format!("{}", v[1] as char).as_str())
            .replace('Y', format!("{}", v[2] as char).as_str())
            .replace('Z', format!("{}", v[3] as char).as_str());

        if s.chars()
            .map(|c| match c {
                'L' => (-1, 0),
                'R' => (1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => unreachable!(),
            })
            .scan((0, 0), |s, v| {
                *s = (s.0 + v.0, s.1 + v.1);
                Some(*s)
            })
            .any(|v| v == (gx, gy))
        {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
