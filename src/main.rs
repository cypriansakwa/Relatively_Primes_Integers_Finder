fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn relatively_prime_elements(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in 1..n {
        if gcd(i, n) == 1 {
            result.push(i);
        }
    }
    result
}

fn main() {
    let n = 48; // Example value
    let primes = relatively_prime_elements(n);
    println!("Elements relatively prime to {}: {:?}", n, primes);
}



