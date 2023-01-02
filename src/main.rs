fn main() {
    println!("Greatest Common Divisor - Euclid's Algorithm");
    let first_number = 128;
    let second_number = 60;
    println!(
        "GCD({}, {}) = {}",
        first_number,
        second_number,
        gcd(first_number, second_number)
    );
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
fn test_gcd_eq() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(64, 28), 4);
}

#[test]
fn test_gcd_ne() {
    assert_ne!(gcd(64, 28), 5);
}
