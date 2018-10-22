fn main() {
    let primes = sieve_of_eratosthenes(200000000);

    for i in &primes {
        println!("> {}", i);
    }

    println!("{} primes found", primes.len());
}


fn sieve_of_eratosthenes(max: usize) -> Vec<u32> {
    // Create sieve array
    let mut sieve = vec![true; max];
    sieve[0] = false;
    sieve[1] = false;

    let end = (max as f64).sqrt() as usize + 1;
    let mut result = Vec::new();

    for i in 2..max {
        if sieve[i as usize] == true {
            if i < end {
                let mut j = (i*i) as usize;
                while j < max {
                    sieve[j] = false;
                    j += i as usize;
                }
            }
            result.push(i as u32);
        }
    }

    result
}
