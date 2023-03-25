use proconio::input;

fn main() {
    input! {k: i64, x: i64, y: i64}
    println!("k: {}, x: {}, y: {}", k, x, y);

    input! {n: i32}

    if n < 0 {
        return;
    }

    println!("{}", n);

    input! {p: [(i64, i64); n as usize]}

    let (mut nx, mut ny) = (0, 0);

    for (tx, ty) in p {
        println!("nx: {}, ny: {}, tx: {}, ty: {} -> (45' nx': {}, ny': {}, tx': {}, ty': {})", nx, ny, tx, ty, nx - ny, nx + ny, tx - ty, tx + ty);
        assert!((tx - nx).abs() + (ty - ny).abs() == k);
        nx = tx;
        ny = ty;
    }

    assert_eq!(x, nx);
    assert_eq!(y, ny);
}