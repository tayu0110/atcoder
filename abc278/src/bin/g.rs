fn main() {
    let mut stdin = proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(n: usize, l: usize, r: usize);

    
}