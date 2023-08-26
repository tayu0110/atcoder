use proconio::*;

fn solve(instructions: &[char], mut c: Vec<Vec<char>>) -> bool {
    let w = c[0].len();
    let mut now = c.last().unwrap().iter().position(|c| *c == 's').unwrap();
    for &nc in instructions {
        match nc {
            'L' => {
                if now == 0 {
                    return false;
                }
                now -= 1;
            }
            'R' => {
                if now == c[0].len() - 1 {
                    return false;
                }
                now += 1;
            }
            _ => {}
        }

        c.pop().unwrap();
        c.insert(0, vec!['.'; w]);

        if c.last().unwrap()[now] == 'x' {
            return false;
        }
    }
    true
}

fn main() {
    input! {h: usize, _: usize, c: [marker::Chars; h]}

    for mut i in 0..3u32.pow(h as u32 - 1) {
        let mut instructions = vec![];
        for _ in 0..h - 1 {
            instructions.push(match i % 3 {
                0 => 'L',
                1 => 'R',
                2 => 'S',
                _ => unreachable!(),
            });
            i /= 3;
        }

        if solve(&instructions, c.clone()) {
            println!("{}", instructions.into_iter().collect::<String>());
            return;
        }
    }

    println!("impossible")
}
