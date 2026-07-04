use proconio::*;

fn main() {
    input! {x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64, x4: i64, y4: i64}

    let a = (y4 - y3) * (x1 - x3) - (x4 - x3) * (y1 - y3);
    let b = (y4 - y3) * (x2 - x3) - (x4 - x3) * (y2 - y3);
    let c = (y2 - y1) * (x3 - x1) - (x2 - x1) * (y3 - y1);
    let d = (y2 - y1) * (x4 - x1) - (x2 - x1) * (y4 - y1);
    let cross_x = ((x2 - x1).abs() + (x4 - x3).abs()) - ((x1 + x2) - (x3 + x4)).abs();
    let cross_y = ((y2 - y1).abs() + (y4 - y3).abs()) - ((y1 + y2) - (y3 + y4)).abs();

    if cross_x >= 0 && cross_y >= 0 {
        if ((a <= 0 && b >= 0) || (a >= 0 && b <= 0)) && ((c <= 0 && d >= 0) || (c >= 0 && d <= 0))
        {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }

    return;
}
