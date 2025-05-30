mod performance;

fn main() {
	println!("Benchmarks");
	let time_ns = performance::benchmark::single_run(|| {
        let _ = (1..=10).fold(1u64, |acc, x| acc * x);
    });
    println!("Single run took: {:.2} ns", time_ns);

	let iterations = performance::benchmark::one_second_run(|| {
        let _ = (1..=10).fold(1u64, |acc, x| acc * x);
    });
    println!("Executed {} times in 1 second", iterations);
}