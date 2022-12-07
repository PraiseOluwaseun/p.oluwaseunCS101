fn main(){
	let p:f64 = 210000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	// simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Depreciation is {}",a);
}