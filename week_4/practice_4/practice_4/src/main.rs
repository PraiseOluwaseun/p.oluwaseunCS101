fn main() {
    let fullname = "Praise Oluwaseun";
    let department = "Computer Science ";
    let uni = "Pan-Altantic university";



    let mut school = "School of Science".to_string();
    //push string

    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    // check lenght

    println!("The lenght of my fullname is: {}", fullname.len());
     println!("I am a student of {}", department);
     println!("{}", school);
     println!("{}", uni);

}
