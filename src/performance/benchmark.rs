#![allow(dead_code)]

use std::time::{Duration, Instant};

pub fn single_run<F>(mut code_block: F) -> f64 
where
	F: FnMut(),
{
	// runs a code block once and
	// returns elapsed time in nanoseconds
	let start = Instant::now();
	code_block();
	let duration = start.elapsed();
	// return duration converted to nanoseconds
	duration.as_secs_f64() * 1e9
}

pub fn timed_run<F>(mut code_block: F, ms: u64) -> u128
where
	F: FnMut(),
{
	// Runs a code block for ms milliseconds (repeat)
	// returns the number of completed repetitions.
	let start = Instant::now();
	let one_sec = Duration::from_millis(ms);
	let mut count = 0u128;

	while start.elapsed() < one_sec {
		code_block();
		count += 1;
	}
	return count;
}