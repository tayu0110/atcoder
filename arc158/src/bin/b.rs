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

    let mut min = Rational::new(std::i64::MAX >> 10, 1);
    let mut max = Rational::new(std::i64::MIN >> 10, 1);
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

        let f = Rational::new(num, den);
        // eprintln!("num: {}", num);
        // eprintln!("den: {}", den);

        if (min.numerator as i128 * f.denominator as i128)
            > min.denominator as i128 * f.numerator as i128
        {
            min = f;
        }
        if (max.numerator as i128 * f.denominator as i128)
            < max.denominator as i128 * f.numerator as i128
        {
            max = f;
        }
    }

    eprintln!("{}", min.numerator);
    eprintln!("{}", min.denominator);
    eprintln!("{}", max.numerator);
    eprintln!("{}", max.denominator);

    println!("{}", min.numerator as f64 / min.denominator as f64);
    println!("{}", max.numerator as f64 / max.denominator as f64);
}
