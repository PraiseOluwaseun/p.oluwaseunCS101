use std::io;

fn main() {
 
 println!("Enter lower bound");
 let mut input1 = String::new();
 io::stdin().read_line(&mut input1).expect("Not a valid input");
 let lower_bound:i32 = input1.trim().parse().expect("fAILDE TO INPUT");

 println!("Enter UPPER bound");
 let mut input2 = String::new();
 io::stdin().read_line(&mut input2).expect("Not a valid input");
 let upper_bound:i32 = input2.trim().parse().expect("fAILDE TO INPUT");

 for x in lower_bound..upper_bound { //upper boung not inclusive
  println!("Count level is {}",x);}

}
