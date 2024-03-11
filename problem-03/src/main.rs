fn prime_factors(mut value: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    let mut factor = 2;

    while value > 1 {
        while value % factor == 0 {
            factors.push(factor);
            value /= factor;
        }

        factor += 1;

        if factor * factor > value {
            if value > 1 {
                factors.push(value)
            }
            break;
        }
    }

    factors
}

fn main() {
    let factors = prime_factors(600851475143);
    let largest_factor = factors.into_iter().max().expect("Number has no prime factors.");

    println!("Largest factor: {largest_factor}")
}
