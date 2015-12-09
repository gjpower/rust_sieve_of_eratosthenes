fn main() {
    println!("Hello, world!");
}




fn sieve_of_eratosthenes(max: i32) -> Vec<bool> {
    // add code here
    let mut sieve = vec![true; max as usize];
    let end = (max as f64).sqrt() as i32 + 1;

    for i in 2..end {
        if sieve[i as usize] == true {
        	for j in i.. {
        	    // add code here
        	}
        }
    }

    sieve
}