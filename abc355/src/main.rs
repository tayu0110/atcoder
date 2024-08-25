use std::io::Write;

use proconio::input_interactive;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let n = rng.gen_range(1..10);
    let l = rng.gen_range(0..(1 << n) - 1);
    let r = rng.gen_range(l..1 << n);

    let arr = (0..1 << n)
        .map(|_| rng.gen_range(0..100))
        .collect::<Vec<_>>();

    println!("{n} {l} {r}");
    std::io::stdout().flush().unwrap();
    for _ in 0..20 {
        input_interactive!(c: char);
        if c == '!' {
            input_interactive!(res: i32);
            assert_eq!(res, arr[l..=r].iter().sum::<i32>() % 100);
            return;
        }
        input_interactive!(i: usize, j: usize);
        let l = (1 << i) * j;
        let r = (1 << i) * (j + 1);
        println!("{}", arr[l..r].iter().sum::<i32>() % 100);
        std::io::stdout().flush().unwrap()
    }
}
