use std::mem::replace;

#[derive(Debug)]
struct Primes {
    current: u32,
    primes: Vec<u32>,
}

impl Primes {
    pub fn new() -> Self {
        Self {
            current: 2,
            primes: Vec::new(),
        }
    }

    fn is_prime(&self, n: u32) -> bool {
        !self.primes.iter().any(|&prime| n % prime == 0)
    }

    fn next_prime(&self) -> u32 {
        if self.current == 2 {
            3
        } else {
            let mut n = self.current + 2;

            while !self.is_prime(n) {
                n += 2;
            }

            n
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.next_prime();

        self.primes.push(next);

        Some(replace(&mut self.current, next))
    }
}

fn main() {
    let mut prime = Primes::new();

    println!("{}", prime.nth(1_000).unwrap());
}
