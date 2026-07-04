use math::MathInt;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {l: usize, r: usize, m: usize}

    let mut ten = 1usize;
    for _ in 0..m {
        ten *= 10;
    }

    let mut now = 5usize.pow_mod(l as u64, ten);
    let mut checked = FxHashMap::default();
    checked.insert(now, l);
    let mut cycle = vec![{
        let mut now = now;
        let mut res = 0;
        while now > 0 {
            res += now % 10;
            now /= 10;
        }
        res
    }];
    let mut start = l;
    for i in l + 1..=r {
        now *= 5;
        now %= ten;
        if let Some(s) = checked.get(&now) {
            start = *s;
            break;
        }
        checked.insert(now, i);
        let mut now = now;
        let mut res = 0;
        while now > 0 {
            res += now % 10;
            now /= 10;
        }
        cycle.push(res);
    }

    let mut diff = r + 1 - l;
    let mut res = cycle[..start - l].iter().sum::<usize>();
    diff -= start - l;
    cycle.drain(..start - l);
    res += cycle.iter().sum::<usize>() * (diff / cycle.len());
    diff %= cycle.len();
    res += cycle[..diff].iter().sum::<usize>();

    println!("{res}")
}
