fn is_prime(n: u32) -> bool {
    // Handle Edge cases
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 || n % 3 == 0 {
        // We have a prime - shortcuts processing for many cases.
        return false;
    }

    // Use the primality test algorithm, not the fastest, but easy to write!
    let mut counter = 5;
    while counter * counter <= n {
        if n % counter == 0 || n % (counter + 2) == 0 {
            return false;
        }
        counter += 6;
    }

    return true;
}

fn find_prime(start_num: u32) -> u32 {
    let mut counter = start_num + 1;

    while ! is_prime(counter) {
        counter += 1;
    }

    return counter;
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    // Add initial value to array
    primes.push(2);

    while primes.len() <= n as usize {
        // primes.push(find_prime(primes.last().unwrap().clone()));
        primes.push(find_prime(*primes.last().unwrap()));
    }

    return *primes.last().unwrap();
}
