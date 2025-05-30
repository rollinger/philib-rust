pub static FACTORIALS: [u128; 35] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
    6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000,
    6402373705728000, 121645100408832000, 2432902008176640000,
    51090942171709440000, 1124000727777607680000, 25852016738884976640000,
    620448401733239439360000, 15511210043330985984000000,
    403291461126605635584000000, 10888869450418352160768000000,
    304888344611713860501504000000, 8841761993739701954543616000000,
    265252859812191058636308480000000, 8222838654177922817725562880000000,
    263130836933693530167218012160000000,
    8683317618811886495518194401280000000,
    295232799039604140847618609643520000000,
];

pub fn factorial_iterative(nb: u128) -> u128 {
	// factorial_iterative(0...35) took 4.587µs.
    let mut nb: u128 = nb;
    let mut f: u128 = 1;
    while nb > 0 {
        f = f * nb;
        nb -= 1;
    }
    return f;
}

pub fn factorial_recursive(nb: u128) -> u128 {
	// factorial_recursive(0...35) took 5.934µs.
	if nb == 0 {
		return 1;
	} else if nb > 0 {
		return nb * factorial_recursive(nb - 1);
	} else {
		return 0;
	}
}

pub fn factorial_std(nb: u128) -> u128 {
	// using std lib. factorial_std(0...35) took 12.495µs
	return (1..=nb).product();
}

pub fn factorial_fake(nb: u128) -> u128 {
	// faking factorials factorial_fake(0...35) took 1.168µs.
	if nb < 35 {
		return FACTORIALS[nb as usize];
	}
	return 0
}

pub fn factorial(number: u8) -> u128 {
	// Calls the fastest algorithm
	// @number: u8: 0..255
	// @return: u128: big
	if number < 35 {
		return factorial_fake(number as u128);
	} else {
		panic!("factorial() only supports values 0..=34, got {number}");
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_initial_test() {
		let nbr_sets = [(0,1),(1,1),(2,2),(3,6),(4,24),(5,120),(6,720),(7,5040),(8,40320),(9,362880),(10,3628800)];
		for set in nbr_sets {
			assert_eq!(factorial_iterative(set.0), set.1);
			assert_eq!(factorial_recursive(set.0), set.1);
			assert_eq!(factorial_std(set.0), set.1);
		}
    }

	#[test]
	fn fake_factorials_equal_iterative() {
		let nbrs = 0..35;
		for nb in nbrs {
			assert_eq!(factorial_iterative(nb), factorial_fake(nb));
		}
	}
}

pub mod benchmarks {
	use super::*;
	use crate::performance::benchmark;

	pub fn bench_factorial_iterative() {
		let single_time_ns = benchmark::single_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_iterative(n);
			}
		});
		let count_in_100ms = benchmark::timed_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_iterative(n);
			}
		}, 100);
		//return "factorial_iterative", single_time_ns, count_in_100ms;
		println!("Bench: factorial_iterative(0...34): single-run {:.2} ns; 100ms-run {} times", single_time_ns, count_in_100ms);
	}

	pub fn bench_factorial_recursive() {
		let single_time_ns = benchmark::single_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_recursive(n);
			}
		});
		let count_in_100ms = benchmark::timed_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_recursive(n);
			}
		}, 100);
		println!("Bench: factorial_recursive(0...34): single-run {:.2} ns; 100ms-run {} times", single_time_ns, count_in_100ms);
	}

	pub fn bench_factorial_std() {
		let single_time_ns = benchmark::single_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_std(n);
			}
		});
		let count_in_100ms = benchmark::timed_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_std(n);
			}
		}, 100);
		println!("Bench: factorial_std(0...34): single-run {:.2} ns; 100ms-run {} times", single_time_ns, count_in_100ms);
	}

	pub fn bench_factorial_fake() {
		let single_time_ns = benchmark::single_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_fake(n);
			}
		});
		let count_in_100ms = benchmark::timed_run(|| {
			let test_numbers = 0..35;
			for n in test_numbers {
				factorial_fake(n);
			}
		}, 100);
		println!("Bench: factorial_fake(0...34): single-run {:.2} ns; 100ms-run {} times", single_time_ns, count_in_100ms);
	}
}
