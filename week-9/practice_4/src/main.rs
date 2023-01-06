use std::fs::OpenOption;
use std::io::Write;

fn main() {
    let mut file = OpenOption::new().append(true).open("data.txt").expect("Failed");
    file.write_all("\nHello Class\nThis is the appendage o the document.".as_bytes()).expect("Write failed");

    println!("File append success");
}
