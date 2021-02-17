use std::time::SystemTime;

fn main() {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("wat");
    let n = now.as_secs() as usize;

    println!("starting at {:?} ({})", now, n);

    let primes = primes_between(n, n+600);
    //let primes = primes_between(50, 101);

    println!("primes:");
    for i in primes {
        println!("{}", i);
    }
}

fn primes_between(start: usize, end: usize) -> Vec<usize> {
    let mut sieve = vec![true; end];

    let limit = (end as f64).sqrt().floor() as usize;

    for i in 2..limit {
        if !sieve[i] {
            continue;
        }

        let mut j = i*i;
        while j < end {
            sieve[j] = false;
            j += i;
        }
    }

    let mut primes = vec![];

    for i in start..end {
        if sieve[i] {
            primes.push(i);
        }
    }

    return primes
}
