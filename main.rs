use std::io;

fn StudentCouncil_VoteX() {
    println!("In this program, you are prompted to answer some questions in order
to check your eligibility to vote.");

    for x in 1..15 {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new();
        let mut input6 = String::new();
        let mut input7 = String::new();
    
        println!("\nWhat is your name? ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let name = input1.trim();
    
        println!("\nWhat is your email address? ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let email = input2.trim();
    
        println!("\nWhat depertment are you in? ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let dep = input3.trim();
    
        println!("\nWhat is your State of Origin? ");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let state = input4.trim();
    
        println!("\nAre you currently a class rep(y or n)? ");
        io::stdin().read_line(&mut input5).expect("Not a valid string");
        let rep = input5.trim();
    
        println!("\nAre you in 200 level or higher(y or n)? ");
        io::stdin().read_line(&mut input6).expect("Not a valid string");
        let level = input6.trim();
    
        println!("\nWhat is your CGPA? ");
        io::stdin().read_line(&mut input7).expect("Not a valid string");
        let cgpa:f64 = input7.trim().parse().expect("Not a valid number");
        
        if (rep == "y" || rep == "Y") & (level == "y" || level == "Y") & (cgpa > 4.0) {
            println!("\nName: {}
Email: {}
Department: {}
State of Origin: {}
\nCongratulations {}, you can vote!",name,email,dep,state,name);
        
        }
        else {
            println!("\nSorry, you are not eligible to vote.");
        }
    
        }
}
fn FacPub() {
    println!("In this program, you are prompted to answer some questions in order
to get the value of your incentive.");
    for x in 1..50 {
        let mut input1 = String::new();
        println!("-----------\nWhat is you name? ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let name = input1.trim();

        let mut input2 = String::new();
        println!("--------\nHow many papers have you published? ");
        io::stdin().read_line(&mut input2).expect("Not a valid number");
        let num:f64 = input2.trim().parse().expect("Not a valid number");

        let mut inc:f64 = 0.0;
        if num >= 3.0 && num <= 5.0 {
            inc = 500000.0;
        }
        else if num > 5.0 && num < 10.0 {
            inc = 800000.0;
        }
        else if inc >= 10.0 {
            inc = 1000000.0;
        }
        else if inc < 3.0 {
            inc = 100000.0;
        }
        println!("----------\nGood day {}, your incentive is N{}", name, inc);
    }
}
fn main() {
    FacPub();
}
