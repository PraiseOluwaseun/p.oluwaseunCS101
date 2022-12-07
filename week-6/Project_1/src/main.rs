use std::io;

fn main() {
    let  mut input1 = String::new;
    println!("Enter name: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");

    println!("Hello {} ", input1);
    println!("\n T = Area of Trapezium formula = height/2*(base1+base2)
 R = Area of the rhombus formula = ½ × diagonal1 × diagonal2
 P = Area of Parallelogram formula = base x altitude
 C = Area of Cube formula = 6 x (length of the side)2 
 Vc = Volume of Cylinder formula = π*radius2 *height");
     
     let  mut input2 = String::new;
    println!("Enter formular: ");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let form = input2.trim();

    if form == "T"{
     trapezium();
    }
    else if form == "R"{
     rhombus();
    }
    else if form == "P"{
     parallelogram();
    }
    else if form == "C"{
     cube();
    }
    else if form == "Cv"{
        cylinder();
    }
    else {
        println!("Please try again");
    }

}

fn trapezium(){
    let mut input1 = String::new;
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut height:i32 = input1.trim().parse().expect("Not a valid input");



    let  mut input2 = String::new;
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let mut base1:i32 = input1.trim().parse().expect("Not a valid input"); 

    let mut input3 = String::new;
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let mut base2:i32 = input2.trim().parse().expect("Not a valid input"); 

    let mut area1:i32 = height / 2 * (base1 + base2);
    println!("Area of Trapezium = {}",area );
    
}
fn rhombus(){
    let  mut input1 = String::new;
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut diagonal1:i32 = input1.trim().parse().expect("Not a valid input");


    let mut input2 = String::new;
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let mut diagonal2:i32 = input2.trim().parse().expect("Not a valid input"); 

    let mut area2:i32 =  1/2 * (diagonal1 * diagonal2);
    println!("Area of Rhombus = {}",area );


}

fn parallelogram(){
    let  mut input1 = String::new;
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut altitude:i32 = input1.trim().parse().expect("Not a valid input");


    let mut  input2 = String::new;
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let mut base:i32 = input2.trim().parse().expect("Not a valid input"); 
    let mut area3:i32 = base * altitude;
    println!("Area of Parallelogram = {}",area );


}
fn cube(){
     let mut input1 = String::new;
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut l:i32 = input1.trim().parse().expect("Not a valid input"); 
    
    let mut area4:i32 = 6 / (l*l);
    println!("Area of Cube = {}",area );
}
fn cylinder(){

    let  mut input1 = String::new;
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut radius2:i32 = input1.trim().parse().expect("Not a valid input");


    let mut input2 = String::new;
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let mut height:i32 = input2.trim().parse().expect("Not a valid input"); 
  
    let mut π = 22 / 7;
    let mut area4:i32 = π*radius2 * height;
    println!("Area of Cylinder = {}",area );


}

