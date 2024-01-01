use proconio::*;

fn main() {
    input! {k: usize}

    let mut now = 0;
    let mut i = 1usize;
    'B: loop {
        let s = i.to_string().chars().collect::<Vec<_>>();
        for (j, v) in s.windows(2).enumerate() {
            if v[0] <= v[1] {
                let mut ten = 1;
                for _ in 0..s.len() - 1 - j {
                    ten *= 10;
                }

                i = (i / ten + 1) * ten;

                continue 'B;
            }
        }

        now += 1;

        if now == k {
            println!("{i}");
            return;
        }

        i += 1;
    }
}
