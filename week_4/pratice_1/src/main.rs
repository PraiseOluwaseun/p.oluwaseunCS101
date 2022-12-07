fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Altantic University";
    let adr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";

    println!("Name:  {}",name);
    println!("University:  {}",uni);

   let department:&'static str = "Computer Science";
   let school:&'static str = "School of Science and Technology";
   println!("Department: {}, \nSchool:{}", department, school);

}
