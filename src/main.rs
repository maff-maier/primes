use std::mem::replace;

#[derive(Debug)]
struct Primes {
    current: u32,
    primes: Vec<u32>,
}

impl Primes {
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

fn primes() -> Primes {
    Primes {
        current: 2,
        primes: Vec::new(),
    }
}

fn main() {
    let mut prime = primes();

    println!("{:#?}", prime);

    println!("{}", prime.nth(10).unwrap());

    println!("{:#?}", prime);

    println!("{}", prime.nth(10_000).unwrap());

    println!("{:#?}", prime);
}
