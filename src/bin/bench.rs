use philib_rust::performance::benchmark;
use philib_rust::factorial::factorial::benchmarks;
// Add more use statements for other benchmarks

fn main() {
	println!("### Benchmarks ###");
	println!("Base Functionality:");
	bench_empty_test_fn();
	bench_pass_test_fn();
	bench_var_assignment();
	bench_iterate_stack_array();
	bench_rand_access_stack_array();
	bench_iterate_heap_array();
	bench_rand_access_heap_array();

    println!("Running all benchmarks:");
    benchmarks::bench_factorial_iterative();
	benchmarks::bench_factorial_recursive();
	benchmarks::bench_factorial_std();
	benchmarks::bench_factorial_fake();
    // Add other benchmarks here


}



fn bench_empty_test_fn() {
	let name: &str = "Bench empty function call";
	fn empty_test_fn(){}
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		empty_test_fn();
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

fn bench_pass_test_fn() {
	let name: &str = "Bench function call with parameter and return";
	fn pass_test_fn(v:u8) -> u8 {v}
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		pass_test_fn(255);
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

fn bench_var_assignment() {
	let name: &str = "Bench variable assignment";
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		let _var: u8 = 255;
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

/*
Stack Array
*/
fn bench_iterate_stack_array() {
	let name: &str = "Bench stack array full iteration on u8*1000";
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		let stack_arr: [u8; 1000] = [0; 1000];
		for i in stack_arr {
			let _value = i;
		}
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

fn bench_rand_access_stack_array() {
	let name: &str = "Bench stack array 1000 random access on u8*1000";
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		use rand::Rng;
		let stack_arr: [u8; 1000] = [0; 1000];
    	let mut rng = rand::thread_rng();

		// Perform 1000 random accesses
		for _ in 0..1000 {
			let index = rng.gen_range(0..stack_arr.len());
			let _value = stack_arr[index];
		}
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

/*
Heap Array
*/
fn bench_iterate_heap_array() {
	let name: &str = "Bench heap array full iteration on u8*1000";
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		let heap_arr: Vec<u8> = vec![0; 1000];
		for i in heap_arr {
			let _value = i;
		}
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}

fn bench_rand_access_heap_array() {
	let name: &str = "Bench heap array 1000 random access on u8*1000";
	let metric: (f64, u128) = benchmark::benchmark_metric(|| {
		use rand::Rng;
		let heap_arr: Vec<u8> = vec![0; 1000];
    	let mut rng = rand::thread_rng();

		// Perform 1000 random accesses
		for _ in 0..1000 {
			let index = rng.gen_range(0..heap_arr.len());
			let _value = heap_arr[index];
		}
	});
	println!("{name}:\navg-100 = {:.2} ns; timed-100ms = {} times.", metric.0, metric.1);
}