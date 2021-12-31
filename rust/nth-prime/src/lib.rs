pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2u32, 3];

    while (primes.len() - 1) < (n as usize) {
        let mut candidate: u32 = *primes.last().unwrap();

        loop {
            candidate += 2;

            let factor = primes.iter().find(|p| candidate % *p == 0);

            if factor.is_none() {
                primes.push(candidate);
                break;
            }
        }
    }

    primes[n as usize]
}
