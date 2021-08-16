// compute the greatest common divisor of two integers
fn main() {
    println!("Hello, world!");
    println!("{}", gcd(12, 18));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(2 * 3 * 7 * 9, 6 * 7 * 2), 42);
}
