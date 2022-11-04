fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();

    let n4 = n1 + &n2 + &n3; // n2 and n3 refrence is passed

    //About E
    println!("\nThe {} is informed by the aspriration to train electrical/electonic engineering professionals in arear of design, building and mauntenance of electrical control systems", n4);

    let w1 = "COmputer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;  

    println!();
    println!("{} is aimed at developing compent, creative, innoviative, entreprenuarial and ethnically-minded persons, capable of crating value in th diverse fields of COmputer Science. ", w3);
}
