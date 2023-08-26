use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    'base: loop {
        let mut t = vec![vec!['.'; 9]; 9];
        for _ in 0..11 {
            loop {
                let r: usize = rng.gen_range(0, 9);
                let c: usize = rng.gen_range(0, 9);
                if t[r][c] == '.' {
                    t[r][c] = '#';
                    break;
                }
            }
        }

        for i in 0..9 {
            for j in 0..9 {
                for (dx, dy) in vec![(0, 1), (1, 0), (1, 1), (!0, 1)] {
                    let (mut r, mut c) = (i, j);
                    let mut ok = false;
                    for _ in 0..7 {
                        if r >= 9 || c >= 9 || t[r][c] == '#' {
                            ok = true;
                            break;
                        }

                        r = r.wrapping_add(dy);
                        c = c.wrapping_add(dx);
                    }

                    if !ok {
                        continue 'base;
                    }
                }
            }
        }

        for v in t {
            println!("{}", v.into_iter().collect::<String>());
        }
        break;
    }
}
