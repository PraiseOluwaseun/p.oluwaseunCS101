use std::mem;

fn main() {
    println!("Hello, world!");
    println!("{} is from {}","Tishe","Lagos");
    println!("{0} is from {1} and {0} is a {2}","Tishe","Lagos","Boss");
     println!("{name} is from {place}.", name = "Tishe", place = "Lagos");
      println!("Binary: {:b}  Hex: {:x} Octal: {:o}", 10, 10, 10);
      println!("{:?}", (12, true, "hello"));

       println!("10 + 10 = {}", 10 + 10);

      // Arrays

       let numbers: [i32;5] = [1,2,3,4,5];
       println!("{:?}", numbers);

       println!("One number: {}", numbers[4]);


       let mut numbers: [i32;5] = [1,2,3,4,5];
       numbers[2] = 22;
       println!("{:?}", numbers);


       println!("Array lenght: {}", numbers.len());


       println!("Array capacity {}  bytes ", mem::size_of_val(&numbers));



       let slice: &[i32] = &numbers[0..2];

       println!("Slice: {:?}", slice);


        // Vectors 
       let mut numbers: Vec<i32> = vec![1,2,3,4,5];
       numbers[2] = 22;

       numbers.push(5);
       numbers.push(6);
       numbers.pop();

       for x in numbers.iter() {
        println!("Numbers: {}", x);
       }  


       println!("{:?}", numbers);


       println!("Vector lenght: {}", numbers.len());


       println!("Vectors capacity {}  bytes ", mem::size_of_val(&numbers));



       let slice: &[i32] = &numbers[0..2];

       println!("Slice: {:?}", slice);


       for x in numbers.iter_mut(){
        *x *= 2;
       }
       println!(" Numbers vec{:?}", numbers);







}
  