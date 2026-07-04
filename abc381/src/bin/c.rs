fn main() {
    proconio::input! {_: usize, s: String}
    println!(
        "{}",
        s.split('/')
            .zip(s.split('/').skip(1))
            .map(|(s, t)| 1
                + (s.len() - s.trim_end_matches('1').len())
                    .min(t.len() - t.trim_start_matches('2').len())
                    * 2)
            .max()
            .unwrap()
    )
}
