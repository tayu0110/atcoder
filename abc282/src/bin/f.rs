use proconio::{input, source::line::LineSource};

fn main() {
    use std::io::{self, *};
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize}

    let mut lr = vec![];
    let mut sparse_table = vec![vec![0; 20]; n + 1];
    for i in 1..=n {
        lr.push((i, i));
        sparse_table[i][0] = lr.len();
        for j in 1..20 {
            if i + (1 << (j - 1)) > n {
                break;
            }

            lr.push((i, i + (1 << (j - 1))));
            sparse_table[i][j] = lr.len();
        }
    }

    println!("{}", lr.len());
    for &(l, r) in &lr {
        println!("{} {}", l, r);
    }

    input! {q: usize}
    for _ in 0..q {
        input! {l: usize, r: usize}

        if l == r {
            println!("{} {}", sparse_table[l][0], sparse_table[l][0]);
            continue;
        }

        let mut j = 0;
        for i in 1..20 {
            if l + (1 << (i - 1)) <= r {
                j = i;
                continue;
            }
            break;
        }

        println!(
            "{} {}",
            sparse_table[l][j],
            sparse_table[r - (1 << (j - 1))][j]
        );
    }
}
