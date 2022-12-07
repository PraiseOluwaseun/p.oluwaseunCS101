fn main(){
	let p:f64 = 520,000000.00;
	let r:f64 = 10.00;
	let t:f64 = 5.00;

	// calculating compand interest

	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}",a);
	let ci = a - p;
	println!("Compound interest is {}", ci);

}