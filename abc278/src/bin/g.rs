fn main() {
    let mut stdin = proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(_n: usize, _l: usize, _r: usize);

    
}