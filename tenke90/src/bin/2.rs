use proconio::input;

fn main() {
    input! {n: usize};

    for i in (0..(1 << n)).rev() {
        let mut buf = vec![0];
        for j in (0..n).rev() {
            let nt = if (i & (1 << j)) != 0 {
                *buf.last().unwrap() + 1
            } else {
                *buf.last().unwrap() - 1
            };
            buf.push(nt);
        }

        if buf.iter().min().unwrap() >= &0 && buf.last().unwrap() == &0 {
            for j in (0..n).rev() {
                if (i & (1 << j)) != 0 {
                    print!("(");
                } else {
                    print!(")");
                }
            }
            println!();
        }
    }
}