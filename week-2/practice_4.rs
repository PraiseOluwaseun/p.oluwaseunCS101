fn main() {
	let p:f64 = 1000.00;
	let r:f64 = 1.00;
	let t:f64 = 2.00;

	// simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}",a);
	let si = a - p;
	println!("Simple interest is {}", si);
}