use proconio::{input, source::line::LineSource};

fn main() {
    let mut stdin = LineSource::new(std::io::BufReader::new(std::io::stdin()));
    input! {from &mut stdin, t: usize};
    let f = {
        let mut f = vec![1, 1];
        for _ in 0..40 {
            f.push(f[f.len()-1] + f[f.len()-2]);
        }
        f
    };

    for _ in 0..t {
        input! {from &mut stdin, mut n: usize};

        let (nn, idx) = {
            let mut nn = 0;
            let mut idx = 0;
            for i in 1..40 {
                if f[i] * 2 + f[i-1] > n {
                    nn = f[i] * 2 + f[i-1];
                    idx = i;
                    break;
                }
            }
            (nn, idx)
        };
        let mut v = vec![-1; nn+1];
        let (mut l, mut m1, mut m2, mut r) = (0, f[idx], f[idx] + f[idx-1], nn);
        v[0] = -1;
        for i in n+1..=nn {
            v[i] = v[i-1] - 1;
        }
        println!("? {}", m1);
        input! {from &mut stdin, na: i32};
        v[m1] = na;
        let mut q = m2;
        loop {
            let na = if q > n {
                v[q]
            } else {
                println!("? {}", q);
                input! {from &mut stdin, na: i32};
                na
            };
            v[q] = na;
            if m1 - l == 1 && r - m2 == 1 {
                println!("! {}", std::cmp::max(v[l], std::cmp::max(v[m1], std::cmp::max(v[m2], v[r]))));
                break;
            }
            if v[m1] < v[m2] {
                l = m1;
                m1 = m2;
                m2 = r - (m1 - l);
                q = m2;
            } else {
                r = m2;
                m2 = m1;
                m1 = l + (r - m2);
                q = m1;
            }
        }
    }
}
