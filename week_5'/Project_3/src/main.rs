use std::io;

fn main() {
    println!("\t     P = Poundo Yam/Edinkaiko Soup          - N3,200
             F = Fried Rice & Chicken               - N3,000
             A = Amala & Ewedu Soup                 - N2,500
             E = Eba & Egusi Soup                   - N2,000
             W = White Rice & Stew                  - N2,500
");
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("Enter order: " );
    io::stdin().read_line(&mut input1).expect("Invaild Order");
    let food = input1.trim();
     
    println!("Enter potion: " );
    io::stdin().read_line(&mut input2).expect("Invaild Order");
    let potion:i32 = input2.trim().parse().expect("Invalid order");

   let mut money:i32 = 0;
   let mut money_new:i32 = 0;

    if food == "P" {
        let money = 3200 * potion;
        println!("You have order Poundo Yam/Edinkaiko Soup   - N3,200"); 
        println!("Your total bill is {}", money);   
    }
    else if food == "A" {
       let money = 2500 * potion;
  
        println!(" You have order Amala & Ewedu Soup  - N2,500");  
        println!("Your total bill is {}", money);  
    }

    else if food == "E" {
        let money = 2000 * potion;
        println!(" You have order Eba & Egusi Soup   - N2,000");
        println!("Your total bill is {}", money);    
    }
    else if food == "F" {
        let money = 3000 * potion;
        println!(" You have order Fried Rice & Chicken  - N3,000");
        println!("Your new total bill is {}", money);

    }

    else if food == "W"{
        let money = 2500 * potion;
        println!(" You have order White Rice & Stew  - N2,500"); 
        println!("Your total bill is {}", money);   
    }
    else {
        println!("Sorry cannot take order");
    }
    if money > 10000 {
        money_new = money * (5/100);
        println!("You have received a discount");
        println!("Your new total bill is {}", money_new)
    }
    


 


}
