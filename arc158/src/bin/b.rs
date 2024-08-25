use proconio::*;
use rational::Rational;

fn main() {
    input! {n: usize, x: [i64; n]}

    let mut pos = vec![];
    let mut neg = vec![];
    for x in x {
        if x > 0 {
            pos.push(x);
        } else {
            neg.push(x);
        }
    }

    pos.sort();
    neg.sort();

    let pos = {
        let mut v = vec![];
        for _ in 0..3 {
            if let Some(k) = pos.pop() {
                v.push(k);
            }
        }
        for i in 0..3 {
            if i < pos.len() {
                v.push(pos[i]);
            }
        }
        v
    };
    let neg = {
        let mut v = vec![];
        for _ in 0..3 {
            if let Some(k) = neg.pop() {
                v.push(k);
            }
        }
        for i in 0..3 {
            if i < neg.len() {
                v.push(neg[i]);
            }
        }
        v
    };

    let t = [pos, neg].concat();

    eprintln!("{:?}", t);

    let mut _min = Rational::<i64>::new(i64::MAX >> 10, 1);
    let mut _max = Rational::<i64>::new(i64::MIN >> 10, 1);
    let len = t.len();
    for i in 1usize..1 << len {
        if i.count_ones() != 3 {
            continue;
        }

        let mut num = 0;
        let mut den = 1;
        for j in 0..len {
            if i & (1 << j) != 0 {
                num += t[j];
                den *= t[j];
            }
        }

        // let f = Rational::<i64>::new(num, den);
        // eprintln!("num: {}", num);
        // eprintln!("den: {}", den);

        // if (min.num as i128 * f.den as i128) > min.den as i128 * f.num as i128 {
        //     min = f;
        // }
        // if (max.num as i128 * f.den as i128) < max.den as i128 * f.num as i128 {
        //     max = f;
        // }
    }

    // eprintln!("{}", min.num);
    // eprintln!("{}", min.den);
    // eprintln!("{}", max.num);
    // eprintln!("{}", max.den);

    // println!("{}", min.num as f64 / min.den as f64);
    // println!("{}", max.num as f64 / max.den as f64);
}
