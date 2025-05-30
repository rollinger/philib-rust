use philib_rust::factorial::factorial::benchmarks;
// Add more use statements for other benchmarks

fn main() {
    println!("Running all benchmarks:");

    benchmarks::bench_factorial_iterative();
	benchmarks::bench_factorial_recursive();
	benchmarks::bench_factorial_std();
	benchmarks::bench_factorial_fake();
    // Add other benchmarks here
}