use std::io; 

fn main() {
    println!("Using quadratic formular");
   let mut a:f32 = 0.0;
   let mut b:f32 = 0.0;
   let mut c:f32 = 0.0;

   let mut rootA:f32 = 0.0;
   let mut rootB:f32 = 0.0;

   let mut disc:f32 = 0.0;

   
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    a = input1.trim().parse().expect("Not a valid number");

    println!("Enter B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    b = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    c = input3.trim().parse().expect("Not a valid number");

    let d = f32::powf(b,2.0);
    let mut roots = d - (4.0 * a * c ) / 2.0 * a;

    if roots > 0.0
    {
        println!("Discriminates:{}",roots)
        println!("There are 2 real roots")

    }
     else if roots == 0.0
    {
        println!("Discriminates:{}",roots)
        println!("There ar no roots")
    }
     if roots < 0.0
    {
        println!("Discriminates:{}",roots)
        println!("There are no real roots")
    }
} 