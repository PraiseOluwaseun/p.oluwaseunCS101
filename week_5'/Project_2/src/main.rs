use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    

    println!("Enter name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter age");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");
    

    println!("Enter years in the company");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let expe:i32 = input3.trim().parse().expect("Not a valid number");

    if expe >= 40 {
        println!("Hello {} your incentive is N1,560,000", input1);
    }
    else if expe >= 30 && expe < 40  {
        println!("Hello {} your incentive is N1,480,000", input1);
    }
    else if  age < 28 {
        println!("Hello {} your incentive is N1,300,000", input1);
    }
    else {
        println!("Hello {} you incentive is 1,000,000",input1);
    }


    
}
