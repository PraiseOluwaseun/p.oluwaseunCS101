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
    

    println!("Enter order:" );
    io::stdin().read_line(&mut input1).expect("Invaild Order");
     
     let P:i32 = 3200;
     let F:i32 = 3000;
     let W:i32 = 2500;
     let E:i32 = 2000;
     let A:i32 = 2500;



    if input1 = P {
        println!("You have order Poundo Yam/Edinkaiko Soup   - N3,200");    
    }
    else if input1:bool = A {
        println!(" You have order Amala & Ewedu Soup  - N2,500");    
    }

    else if input1 = E {
        println!(" You have order Eba & Egusi Soup   - N2,000");    
    }
    else if input1 =  F {
        println!(" You have order Fried Rice & Chicken  - N3,000");    
    }

    else if input1 = W {
        println!(" You have order White Rice & Stew  - N2,500");    
    }

 println!("Enter potion:" );
    io::stdin().read_line(&mut input2).expect("Invaild Order");
    let potion:i32 = input2.trim().parse().expect("Invalid order");

  let order = potion * input1;

  println!("Your total bill is {}", order)


}
