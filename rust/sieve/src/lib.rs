struct SieveOfEratosthenes {
    current: usize,
    sieve: Vec<bool>,
}

impl SieveOfEratosthenes {
    fn new(upper_bound: u64) -> Self {
        let mut sieve = vec![true; (upper_bound + 1) as usize];

        sieve[0] = false;
        sieve[1] = false;

        let mut p: u64 = 2;
        while (p * p) <= upper_bound {
            if sieve[p as usize] {
                for i in (p * p..upper_bound + 1).step_by(p as usize) {
                    sieve[i as usize] = false;
                }
            }
            p += 1;
        }

        Self {
            current: 0,
            sieve: sieve,
        }
    }
}

impl Iterator for SieveOfEratosthenes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current != self.sieve.len() {
            if self.sieve[self.current] {
                let r = self.current as u64;
                self.current += 1;
                return Some(r);
            }
            self.current += 1;
        }
        return None;
    }
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    SieveOfEratosthenes::new(upper_bound).collect::<Vec<u64>>()
}
