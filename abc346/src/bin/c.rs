use iolib::*;

fn main() {
    scan!(n: usize, k: usize, mut a: [u32; n]);

    a.sort_unstable();
    a.dedup();

    putln!(
        k * (k + 1) / 2
            - a.into_iter()
                .filter_map(|a| (a <= k as u32).then_some(a as usize))
                .sum::<usize>()
    );
}
