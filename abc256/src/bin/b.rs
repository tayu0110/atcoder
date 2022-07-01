use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut now = vec![0; 4];
    let mut res = 0;
    for v in a {
        let mut to = vec![0; 4];
        now[0] += 1;
        for i in 0..4 {
            let j = i + v;
            if j > 3 {
                res += now[i]
            } else {
                to[j] += now[i];
            }
        }

        std::mem::swap(&mut now, &mut to);
    }

    println!("{}", res);
}