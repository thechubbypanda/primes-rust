use std::time::SystemTime;
use std::env;

fn main() {
	let end = 10_000;
	let mut primes = vec![true; end];
	primes[0] = false;
	primes[1] = false;

	let start = SystemTime::now();

	for n in 2..end {
		for multiple in ((n + n)..end).step_by(n) {
			primes[multiple] = false;
		}
	}

	let time = SystemTime::now().duration_since(start).unwrap();

	// Print primes if "print" arg passed
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 && args[1].eq(&String::from("print")) {
		for n in 0..primes.len() {
			if primes[n] {
				println!("{}", n);
			}
		}
	}

	println!("Time taken: {} seconds", time.as_secs_f64());
}
