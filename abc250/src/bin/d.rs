use proconio::*;

struct Sieve<const MAX: usize = 1000010> {
    count: usize,
    primes: [usize; MAX],
}

impl<const MAX: usize> Sieve<MAX> {
    const fn new() -> Self {
        let mut primes = [usize::MAX; MAX];
        let mut count = 0;

        let mut i = 2;
        while i < MAX {
            if primes[i] == usize::MAX {
                primes[i] = i;
                primes[count] = i;
                count += 1;
            }

            let mut j = 0;
            while j < count {
                if primes[j] > primes[i] || primes[j] * i >= MAX {
                    break;
                }
                primes[primes[j] * i] = primes[j];
                j += 1;
            }

            i += 1;
        }

        Self { count, primes }
    }

    const fn count(&self) -> usize {
        self.count
    }
}

impl std::ops::Index<usize> for Sieve {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.primes[index]
    }
}
const PRIMES: Sieve = Sieve::new();

fn main() {
    input! {n: usize}

    let mut res = 0;
    for i in 0..PRIMES.count() {
        let tri = PRIMES[i] * PRIMES[i] * PRIMES[i];
        let (mut l, mut r) = (-1, i as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if tri.saturating_mul(PRIMES[m as usize]) <= n {
                l = m;
            } else {
                r = m;
            }
        }
        res += r as usize;
    }

    println!("{res}")
}
