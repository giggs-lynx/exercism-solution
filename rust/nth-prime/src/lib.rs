pub fn nth(n: u32) -> Option<u32> {

    if n == 0 {
        return None
    }

    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);

    while primes.len() < (n as usize) {
        let last_prime = primes[primes.len() - 1];

        let mut candidate = (last_prime / 2) * 2 + 1;
        loop {

            let mut is_prime: bool = true;
            for p in &primes {
                if candidate % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            
            if is_prime {
                primes.push(candidate);
                break;
            } else {
                candidate += 2;
            }
        }
    }

    Some(primes[primes.len() - 1])
}