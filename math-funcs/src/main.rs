// most of these likely exist. just doing for fun.

fn main() {
    assert_eq!(max(33, 44), 44);
    assert_eq!(min(33, 44), 33);
    assert_eq!(floor(5.6), 5);
    assert_eq!(round(43.67), 44);
}

fn max(x: i64, y: i64) -> i64 {
    if x > y { x } else { y } // no ternary operator, sadge.
}

fn min(x: i64, y: i64) -> i64 {
    if x < y { x } else { y }
}

fn floor(x: f64) -> i64 {
    (x - x % 1.0) as i64
}

fn round(x: f64) -> i64 {
    if x % 1.0 < 0.5 { (x - x % 1.0) as i64 } else { (x + (1.0 - (x % 1.0))) as i64 } // inner brackets not needed, just easier to read like that.
}